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
) -> Result<Value> {
    Ok(match element.name.to_lowercase().as_str() {
        "program" if depth == 0 => element.children.iter().try_fold(Value::Null, |_, child| {
            interpret(child, depth + 1, variables)
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
                    let child_value = interpret(child, depth + 1, variables)?;

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
            let value = interpret(child, depth + 1, variables)?;

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
            let value = interpret(child, depth + 1, variables)?;

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
            let value = interpret(child, depth + 1, variables)?;

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
                let value = interpret(child, depth + 1, variables)?;
                output.push_str(&value.to_string());
            }

            if newline {
                println!("{output}");
            } else {
                print!("{output}");
            }

            output.into()
        }

        name @ ("unwrap" | "expect") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables)?;

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
                        let child_value = interpret(child, depth + 1, variables)?;

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
                } else {
                    ensure!(
                        element.children.len() == 1,
                        "Expected exactly one child or the `var` attribute in <get> element"
                    );

                    let child = &element.children[0];
                    interpret(child, depth + 1, variables)?
                }
            } else {
                ensure!(
                    element.children.len() == 1,
                    "Expected exactly one child or the `var` attribute in <get> element"
                );

                let child = &element.children[0];
                let value = interpret(child, depth + 1, variables)?;

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
            let value = interpret(child, depth + 1, variables)?;

            variables.insert(name, value.clone());

            value
        }

        "add" | "sum" => element
            .children
            .iter()
            .map(|child| interpret(child, depth + 1, variables))
            .sum::<Result<Value>>()?,

        name @ ("neg" | "negate" | "negative") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables)?;

            (-value)?
        }

        "not" => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <not> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables)?;

            (!value)?
        }

        name @ ("abs" | "absolute") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables)?;

            value.abs()?
        }

        "sub" | "subtract" | "difference" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc - value)?
        }

        "mul" | "multiply" | "product" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc * value)?
        }

        "div" | "divide" | "quotient" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc / value)?
        }

        _ => bail!("Unknown element: {}", element.name),
    })
}
