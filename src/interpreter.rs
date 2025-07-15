use miette::{Context, Report, Result, bail, ensure};

use crate::{element::Element, value::Value};

// TODO: include code snippets with errors
pub fn interpret(element: &Element, depth: u32) -> Result<Value> {
    Ok(match element.name.to_lowercase().as_str() {
        "program" if depth == 0 => element
            .children
            .iter()
            .try_fold(Value::Null, |_, child| interpret(child, depth + 1))?,
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

        "null" => Value::Null,

        name @ ("int" | "integer") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1)?;

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
            let value = interpret(child, depth + 1)?;

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
            let value = interpret(child, depth + 1)?;

            value.as_bool().into()
        }

        "print" => {
            let newline = element
                .attributes
                .get("newline")
                .map(|s| Value::from(s.as_str()).as_bool())
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

        name @ ("unwrap" | "expect") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1)?;

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
                        let child_value = interpret(child, depth + 1)?;

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

        "add" | "sum" => element
            .children
            .iter()
            .map(|child| interpret(child, depth + 1))
            .sum::<Result<Value>>()?,

        "mul" | "multiply" | "product" => element
            .children
            .iter()
            .map(|child| interpret(child, depth + 1))
            .product::<Result<Value>>()?,

        _ => bail!("Unknown element: {}", element.name),
    })
}
