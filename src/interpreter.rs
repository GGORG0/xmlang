use std::str::FromStr;

use miette::{Report, Result, bail};

use crate::{element::Element, value::Value};

// TODO: include code snippets with errors
pub fn interpret(element: &Element, depth: u32) -> Result<Value> {
    Ok(match element.name.to_lowercase().as_str() {
        "program" if depth == 0 => {
            let mut last_value = Value::Null;
            for child in &element.children {
                last_value = interpret(child, depth + 1)?;
            }
            last_value
        }
        _ if depth == 0 => bail!("Root element must be <program>"),

        "space" => {
            let count = element
                .attributes
                .get("count")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let spaces = " ".repeat(count);
            Value::Str(spaces)
        }

        "" | "str" | "string" => {
            let text = element.children.iter().try_fold(
                element.attributes.get("_text").cloned().unwrap_or_default(),
                |value, child| {
                    let child_value = interpret(child, depth + 1)?;

                    Ok::<_, Report>(if child_value.is_null() {
                        value
                    } else {
                        format!("{value}{child_value}")
                    })
                },
            )?;
            Value::Str(text)
        }

        "print" => {
            let newline = element
                .attributes
                .get("newline")
                .and_then(|s| Value::from_str(s).unwrap().as_bool())
                .unwrap_or(true);

            let mut output = String::new();
            for child in &element.children {
                let value = interpret(child, depth + 1)?;
                output.push_str(&value.to_string());
            }

            if newline {
                println!("{output}");
            } else {
                print!("{output}");
            }

            Value::Null
        }

        _ => bail!("Unknown element: {}", element.name),
    })
}
