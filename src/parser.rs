use std::{collections::HashMap, path::Path};

use color_eyre::{Report, Result, eyre::bail};
use quick_xml::{
    Decoder, NsReader,
    events::{BytesStart, Event},
    name::ResolveResult,
};

use crate::element::Element;

impl Element {
    fn from_event(ns: ResolveResult, e: BytesStart, decoder: &Decoder) -> Result<Self> {
        let ns = match ns {
            ResolveResult::Bound(ns) => {
                let ns = decoder.decode(ns.as_ref())?;
                if ns.is_empty() {
                    None
                } else {
                    Some(ns.to_string())
                }
            }
            ResolveResult::Unbound => None,
            ResolveResult::Unknown(ns) => {
                let ns = decoder.decode(ns.as_ref())?;
                bail!("Unknown namespace: {}", ns);
            }
        };

        let name = e.name().local_name();
        let name = decoder.decode(name.as_ref())?;

        let attributes = e
            .attributes()
            .map(|attr| {
                let attr = attr?;
                let key = decoder.decode(attr.key.as_ref())?;
                let value = decoder.decode(attr.value.as_ref())?;
                Ok((key.to_string(), value.to_string()))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self {
            namespace: ns,
            name: name.to_string(),
            attributes,
            children: Vec::new(),
        })
    }
}

pub fn parse(path: &Path) -> Result<Element> {
    let mut reader = NsReader::from_file(path)?;
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let decoder = reader.decoder();

    let mut stack: Vec<Element> = Vec::new();

    loop {
        match reader.read_resolved_event_into(&mut buf) {
            Ok((ns, Event::Start(e))) => {
                stack.push(Element::from_event(ns, e, &decoder)?);
            }

            Ok((ns, Event::Empty(e))) => {
                let element = Element::from_event(ns, e, &decoder)?;

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(element);
                } else {
                    return Ok(element);
                }
            }

            Ok((ResolveResult::Unbound, Event::Text(e))) => {
                let element = Element {
                    namespace: None,
                    name: String::new(),
                    attributes: HashMap::from([(
                        "text-content".to_string(),
                        decoder.decode(e.as_ref())?.to_string(),
                    )]),
                    children: Vec::new(),
                };

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(element);
                } else {
                    return Ok(element);
                }
            }

            Ok((_, Event::End(_))) => {
                let completed_element = stack.pop().ok_or_else(|| {
                    Report::msg("Unexpected end tag without a matching start tag")
                })?;

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(completed_element);
                } else {
                    return Ok(completed_element);
                }
            }

            Ok((ResolveResult::Unbound, Event::Eof)) => break,

            Err(e) => bail!("Error at position {}: {:?}", reader.error_position(), e),

            ev => {
                println!("Other event: {ev:?}");
            }
        }
        buf.clear();
    }

    while stack.len() > 1 {
        let completed_element = stack.pop().unwrap();

        if let Some(parent) = stack.last_mut() {
            parent.children.push(completed_element);
        } else {
            return Ok(completed_element);
        }
    }

    if let Some(root) = stack.pop() {
        Ok(root)
    } else {
        bail!("No root element found")
    }
}
