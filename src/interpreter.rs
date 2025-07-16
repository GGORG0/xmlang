use std::collections::HashMap;

use miette::{Context, Report, Result, bail, ensure};

use crate::{
    element::Element,
    value::{Abs, Value},
};

// TODO: include code snippets with errors
pub fn interpret(
    element: &Element,
    depth: u32,
    variables: &mut HashMap<String, Value>,
    specials: &[HashMap<String, Value>],
) -> Result<Value> {
    Ok(match element.name.to_lowercase().as_str() {
        "program" if depth == 0 => element.children.iter().try_fold(Value::Null, |_, child| {
            interpret(child, depth + 1, variables, specials)
        })?,
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
                    let child_value = interpret(child, depth + 1, variables, specials)?;

                    Ok::<_, Report>(if child_value.is_null() {
                        value
                    } else {
                        format!("{value}{child_value}")
                    })
                },
            )?;
            Value::Str(text)
        }

        "null" => Value::Null,

        name @ ("int" | "integer") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            value
                .as_int()
                .wrap_err("Failed to convert value to an integer")?
                .into()
        }

        "float" => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <float> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            value
                .as_float()
                .wrap_err("Failed to convert value to a float")?
                .into()
        }

        "bool" => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <bool> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            value.as_bool().into()
        }

        "type" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials))
                .collect::<Result<Vec<Value>>>()?;

            let types = values
                .into_iter()
                .map(|value| value.type_name())
                .collect::<Vec<_>>();

            Value::Str(if types.is_empty() {
                Value::Null.type_name()
            } else {
                types.join(" ")
            })
        }

        "print" => {
            let newline = element
                .attributes
                .get("newline")
                .map(|s| Value::from(s.as_str()).as_bool())
                .unwrap_or(true);

            let mut output = String::new();
            for child in &element.children {
                let value = interpret(child, depth + 1, variables, specials)?;
                output.push_str(&value.to_string());
            }

            if newline {
                println!("{output}");
            } else {
                print!("{output}");
            }

            output.into()
        }

        "join" => {
            let separator = element
                .attributes
                .get("separator")
                .cloned()
                .unwrap_or_else(|| " ".to_string());

            let start = element.attributes.get("start").cloned().unwrap_or_default();

            let end = element.attributes.get("end").cloned().unwrap_or_default();

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials))
                .collect::<Result<Vec<Value>>>()?;

            let joined = values
                .into_iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(&separator);

            Value::Str([start, joined, end].concat())
        }

        name @ ("unwrap" | "expect") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            if value.is_null() {
                let msg = element
                    .attributes
                    .get("message")
                    .cloned()
                    .unwrap_or_else(|| "Unwrapped value is null".to_string());

                bail!(msg);
            }

            value
        }

        "throw" => {
            if let Some(msg) = element.attributes.get("message").cloned() {
                bail!(msg);
            } else {
                let text = element.children.iter().try_fold(
                    element.attributes.get("_text").cloned().unwrap_or_default(),
                    |value, child| {
                        let child_value = interpret(child, depth + 1, variables, specials)?;

                        Ok::<_, Report>(if child_value.is_null() {
                            value
                        } else {
                            format!("{value}{child_value}")
                        })
                    },
                )?;

                if text.is_empty() {
                    bail!("An error occurred, but no message was provided.");
                } else {
                    bail!(text);
                }
            }
        }

        "get" => {
            if let Some(name) = element.attributes.get("var") {
                if let Some(var) = variables.get(name).cloned() {
                    var
                } else if element.children.is_empty() {
                    Default::default()
                } else {
                    ensure!(
                        element.children.len() == 1,
                        "Expected exactly one child or the `var` attribute in <get> element"
                    );

                    let child = &element.children[0];
                    interpret(child, depth + 1, variables, specials)?
                }
            } else {
                ensure!(
                    element.children.len() == 1,
                    "Expected exactly one child or the `var` attribute in <get> element"
                );

                let child = &element.children[0];
                let value = interpret(child, depth + 1, variables, specials)?;

                variables
                    .get(&value.to_string())
                    .cloned()
                    .unwrap_or_default()
            }
        }

        "set" => {
            let name = element
                .attributes
                .get("var")
                .wrap_err("Expected the `var` attribute in <set> element")?
                .clone();

            ensure!(
                element.children.len() == 1,
                "Expected exactly one child or the `var` attribute in <get> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            variables.insert(name, value.clone());

            value
        }

        "special" => {
            if let Some(name) = element.attributes.get("name") {
                specials
                    .iter()
                    .find_map(|specials_map| specials_map.get(name).cloned())
                    .wrap_err(format!("Special `{name}` not found"))?
            } else {
                ensure!(
                    element.children.len() == 1,
                    "Expected exactly one child or the `name` attribute in <special> element"
                );

                let child = &element.children[0];
                let value = interpret(child, depth + 1, variables, specials)?;

                let name = value.to_string();

                specials
                    .iter()
                    .find_map(|specials_map| specials_map.get(&name).cloned())
                    .wrap_err(format!("Special `{name}` not found"))?
            }
        }

        "add" | "sum" => element
            .children
            .iter()
            .map(|child| interpret(child, depth + 1, variables, specials))
            .sum::<Result<Value>>()?,

        name @ ("neg" | "negate" | "negative") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            (-value)?
        }

        "not" => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <not> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            (!value)?
        }

        name @ ("abs" | "absolute") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials)?;

            value.abs()?
        }

        "sub" | "subtract" | "difference" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc - value)?
        }

        "mul" | "multiply" | "product" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc * value)?
        }

        "div" | "divide" | "quotient" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc / value)?
        }

        "try" => {
            ensure!(
                element.children.len() == 2,
                "Expected exactly 2 children in <try> element"
            );

            let do_block = element
                .children
                .iter()
                .find(|child| child.name.to_lowercase() == "do")
                .wrap_err("Expected a <do> child in <try> element")?;

            let catch_block = element
                .children
                .iter()
                .find(|child| child.name.to_lowercase() == "catch")
                .wrap_err("Expected a <catch> child in <try> element")?;

            let ret = do_block.children.iter().try_fold(Value::Null, |_, child| {
                interpret(child, depth + 1, variables, specials)
            });

            ret.or_else(|err| {
                let err = Value::from(err.to_string());

                let specials = [&[HashMap::from([("error".to_string(), err)])], specials].concat();

                catch_block
                    .children
                    .iter()
                    .try_fold(Value::Null, |_, child| {
                        interpret(child, depth + 1, variables, &specials)
                    })
            })?
        }

        "block" => element.children.iter().try_fold(Value::Null, |_, child| {
            interpret(child, depth + 1, variables, specials)
        })?,

        _ => bail!("Unknown element: {}", element.name),
    })
}
