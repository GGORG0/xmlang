use std::{
    collections::HashMap,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use miette::{Context, Diagnostic, IntoDiagnostic, Report, Result, bail, ensure};
use thiserror::Error;

use crate::{
    element::Element,
    value::{Abs, Value},
};

// hacky!
#[derive(Debug, Clone, Error, Diagnostic)]
enum BlockControl {
    #[error("")] // this should never be seen by the user
    Break(Value),
    #[error("Tried to continue outside of a loop")]
    Continue,
}

impl Default for BlockControl {
    fn default() -> Self {
        Self::Break(Value::Null)
    }
}

// TODO: include code snippets with errors
pub fn interpret(
    element: &Element,
    depth: u32,
    variables: &mut HashMap<String, Value>,
    specials: &[HashMap<String, Value>],
    functions: &mut HashMap<String, Vec<Element>>,
) -> Result<Value> {
    Ok(match element.name.to_lowercase().as_str() {
        "program" if depth == 0 => {
            match element.children.iter().try_fold(Value::Null, |_, child| {
                interpret(child, depth + 1, variables, specials, functions)
            }) {
                Ok(val) => val,
                Err(err) => match err.downcast::<BlockControl>() {
                    Ok(BlockControl::Break(val)) => val,
                    Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),
                    Err(err) => return Err(err),
                },
            }
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
                    let child_value = interpret(child, depth + 1, variables, specials, functions)?;

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
            let value = interpret(child, depth + 1, variables, specials, functions)?;

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
            let value = interpret(child, depth + 1, variables, specials, functions)?;

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
            let value = interpret(child, depth + 1, variables, specials, functions)?;

            value.as_bool().into()
        }

        "true" => Value::Bool(true),
        "false" => Value::Bool(false),

        "type" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
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

        name @ ("delay" | "sleep") => {
            let duration = if let Some(duration) = element
                .attributes
                .get("duration")
                .and_then(|s| s.parse::<u64>().ok())
            {
                duration
            } else {
                ensure!(
                    element.children.len() == 1,
                    "Expected exactly one child or the `duration` attribute in <{name}> element"
                );

                let child = &element.children[0];
                let value = interpret(child, depth + 1, variables, specials, functions)?;

                value
                    .as_int()
                    .wrap_err("Failed to convert value to an integer")? as u64
            };

            sleep(Duration::from_millis(duration));

            Value::Null
        }

        "print" => {
            let newline = element
                .attributes
                .get("newline")
                .map(|s| Value::from(s.as_str()).as_bool())
                .unwrap_or(true);

            let mut output = String::new();
            for child in &element.children {
                let value = interpret(child, depth + 1, variables, specials, functions)?;
                output.push_str(&value.to_string());
            }

            if newline {
                println!("{output}");
            } else {
                print!("{output}");
                io::stdout().flush().into_diagnostic()?;
            }

            output.into()
        }

        "readline" => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).into_diagnostic()?;

            Value::Str(input.trim_end_matches(['\r', '\n']).to_string())
        }

        "trim" => {
            let start = element
                .attributes
                .get("start")
                .map(|s| Value::from(s.as_str()).as_bool())
                .unwrap_or(true);

            let end = element
                .attributes
                .get("end")
                .map(|s| Value::from(s.as_str()).as_bool())
                .unwrap_or(true);

            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <trim> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials, functions)?;

            let value = value.to_string();

            (if start && end {
                value.trim()
            } else if start {
                value.trim_start()
            } else if end {
                value.trim_end()
            } else {
                &value
            })
            .into()
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
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
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
            let value = interpret(child, depth + 1, variables, specials, functions)?;

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
                        let child_value =
                            interpret(child, depth + 1, variables, specials, functions)?;

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

        name @ ("return" | "break") => {
            ensure!(
                element.children.len() <= 1,
                "Expected at most one child in <{name}> element"
            );

            let value = if element.children.is_empty() {
                Value::Null
            } else {
                let child = &element.children[0];
                interpret(child, depth + 1, variables, specials, functions)?
            };

            return Err(BlockControl::Break(value).into());
        }

        name @ ("continue" | "next") => {
            ensure!(
                element.children.is_empty(),
                "Expected no children in <{name}> element"
            );

            return Err(BlockControl::Continue.into());
        }

        "exit" => {
            ensure!(
                element.children.is_empty(),
                "Expected no children in <exit> element"
            );

            let code = element
                .attributes
                .get("code")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);

            std::process::exit(code);
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
                    interpret(child, depth + 1, variables, specials, functions)?
                }
            } else {
                ensure!(
                    element.children.len() == 1,
                    "Expected exactly one child or the `var` attribute in <get> element"
                );

                let child = &element.children[0];
                let value = interpret(child, depth + 1, variables, specials, functions)?;

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
            let value = interpret(child, depth + 1, variables, specials, functions)?;

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
                let value = interpret(child, depth + 1, variables, specials, functions)?;

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
            .map(|child| interpret(child, depth + 1, variables, specials, functions))
            .sum::<Result<Value>>()?,

        name @ ("neg" | "negate" | "negative") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials, functions)?;

            (-value)?
        }

        "not" => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <not> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials, functions)?;

            (!value)?
        }

        "and" => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <and> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_true = values.iter().all(|value| value.as_bool());
            Value::Bool(all_true)
        }

        "or" => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <or> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let any_true = values.iter().any(|value| value.as_bool());
            Value::Bool(any_true)
        }

        name @ ("abs" | "absolute") => {
            ensure!(
                element.children.len() == 1,
                "Expected exactly one child in <{name}> element"
            );

            let child = &element.children[0];
            let value = interpret(child, depth + 1, variables, specials, functions)?;

            value.abs()?
        }

        "sub" | "subtract" | "difference" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc - value)?
        }

        "mul" | "multiply" | "product" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc * value)?
        }

        "div" | "divide" | "quotient" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc / value)?
        }

        "mod" | "modulo" | "remainder" => {
            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let mut values = values.into_iter();

            let first = values.next().unwrap_or_default();
            values.try_fold(first, |acc, value| acc % value)?
        }

        name @ ("eq" | "equals" | "equal") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_equal = values.windows(2).all(|w| w[0] == w[1]);
            Value::Bool(all_equal)
        }

        name @ ("ne" | "not-equals" | "not-equal") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_not_equal = values.windows(2).all(|w| w[0] != w[1]);
            Value::Bool(all_not_equal)
        }

        name @ ("lt" | "less-than") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_less_than = values.windows(2).all(|w| w[0] < w[1]);

            Value::Bool(all_less_than)
        }

        name @ ("le" | "less-than-or-equal") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_less_than_or_equal = values.windows(2).all(|w| w[0] <= w[1]);

            Value::Bool(all_less_than_or_equal)
        }

        name @ ("gt" | "greater-than") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_greater_than = values.windows(2).all(|w| w[0] > w[1]);

            Value::Bool(all_greater_than)
        }

        name @ ("ge" | "greater-than-or-equal") => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <{name}> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let all_greater_than_or_equal = values.windows(2).all(|w| w[0] >= w[1]);

            Value::Bool(all_greater_than_or_equal)
        }

        "starts-with" => {
            ensure!(
                element.children.len() == 2,
                "Expected exactly 2 children in <starts-with> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let left = &values[0];
            let right = &values[1];

            Value::Bool(left.to_string().starts_with(&right.to_string()))
        }

        "ends-with" => {
            ensure!(
                element.children.len() == 2,
                "Expected exactly 2 children in <ends-with> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let left = &values[0];
            let right = &values[1];

            Value::Bool(left.to_string().ends_with(&right.to_string()))
        }

        "contains" => {
            ensure!(
                element.children.len() == 2,
                "Expected exactly 2 children in <contains> element"
            );

            let values = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let left = &values[0];
            let right = &values[1];

            Value::Bool(left.to_string().contains(&right.to_string()))
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
                interpret(child, depth + 1, variables, specials, functions)
            });

            match ret {
                Ok(val) => val,
                Err(err) => match err.downcast::<BlockControl>() {
                    Ok(BlockControl::Break(val)) => val,
                    Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),

                    Err(err) => {
                        let err_val = Value::from(err.to_string());
                        let specials =
                            [&[HashMap::from([("error".to_string(), err_val)])], specials].concat();

                        match catch_block
                            .children
                            .iter()
                            .try_fold(Value::Null, |_, child| {
                                interpret(child, depth + 1, variables, &specials, functions)
                            }) {
                            Ok(val) => val,
                            Err(err) => match err.downcast::<BlockControl>() {
                                Ok(BlockControl::Break(val)) => val,
                                Ok(BlockControl::Continue) => {
                                    return Err(BlockControl::Continue.into());
                                }
                                Err(err) => return Err(err),
                            },
                        }
                    }
                },
            }
        }

        "block" => {
            match element.children.iter().try_fold(Value::Null, |_, child| {
                interpret(child, depth + 1, variables, specials, functions)
            }) {
                Ok(val) => val,
                Err(err) => match err.downcast::<BlockControl>() {
                    Ok(BlockControl::Break(val)) => val,
                    Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),
                    Err(err) => return Err(err),
                },
            }
        }

        "if" => {
            ensure!(
                element.children.len() >= 2,
                "Expected at least 2 children in <if> element"
            );

            let mut condition_count = 0;
            let mut then_count = 0;
            let mut else_count = 0;

            for child in element.children.iter() {
                let key = child.name.to_lowercase();
                match key.as_str() {
                    "condition" => condition_count += 1,
                    "then" => then_count += 1,
                    "elif" => {}
                    "else" => else_count += 1,
                    _ => bail!("Unexpected child in <if> element: {}", child.name),
                }
            }

            ensure!(
                condition_count == 1,
                "Expected exactly one <condition> child in <if> element"
            );
            ensure!(
                then_count == 1,
                "Expected exactly one <then> child in <if> element"
            );
            ensure!(
                else_count <= 1,
                "Expected at most one <else> child in <if> element"
            );

            let condition = element
                .children
                .iter()
                .find(|child| child.name.to_lowercase() == "condition")
                .wrap_err("Expected a <condition> child in <if> element")?;

            let then_block = element
                .children
                .iter()
                .find(|child| child.name.to_lowercase() == "then")
                .wrap_err("Expected a <then> child in <if> element")?;

            let elif_blocks: Vec<&Element> = element
                .children
                .iter()
                .filter(|child| child.name.to_lowercase() == "elif")
                .collect();

            let else_block = element
                .children
                .iter()
                .find(|child| child.name.to_lowercase() == "else");

            for elif_block in &elif_blocks {
                ensure!(
                    elif_block.children.len() == 2,
                    "Expected exactly 2 children in <elif> element"
                );

                let mut elif_condition_count = 0;
                let mut elif_then_count = 0;

                for child in elif_block.children.iter() {
                    let key = child.name.to_lowercase();
                    match key.as_str() {
                        "condition" => elif_condition_count += 1,
                        "then" => elif_then_count += 1,
                        _ => bail!("Unexpected child in <elif> element: {}", child.name),
                    }
                }

                ensure!(
                    elif_condition_count == 1,
                    "Expected exactly one <condition> child in <elif> element"
                );
                ensure!(
                    elif_then_count == 1,
                    "Expected exactly one <then> child in <elif> element"
                );
            }

            ensure!(
                condition.children.len() == 1,
                "Expected exactly one child in <condition> element"
            );
            let condition_value = interpret(
                &condition.children[0],
                depth + 2,
                variables,
                specials,
                functions,
            )?;

            let specials = [
                &[HashMap::from([(
                    "condition".to_string(),
                    condition_value.clone(),
                )])],
                specials,
            ]
            .concat();

            if condition_value.as_bool() {
                match then_block
                    .children
                    .iter()
                    .try_fold(Value::Null, |_, child| {
                        interpret(child, depth + 1, variables, &specials, functions)
                    }) {
                    Ok(val) => val,
                    Err(err) => match err.downcast::<BlockControl>() {
                        Ok(BlockControl::Break(val)) => val,
                        Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),
                        Err(err) => return Err(err),
                    },
                }
            } else {
                for elif_block in &elif_blocks {
                    let elif_condition = elif_block
                        .children
                        .iter()
                        .find(|child| child.name.to_lowercase() == "condition")
                        .unwrap();

                    let elif_then = elif_block
                        .children
                        .iter()
                        .find(|child| child.name.to_lowercase() == "then")
                        .unwrap();

                    ensure!(
                        elif_condition.children.len() == 1,
                        "Expected exactly one child in <condition> element"
                    );
                    let elif_condition_value = interpret(
                        &elif_condition.children[0],
                        depth + 2,
                        variables,
                        &specials,
                        functions,
                    )?;

                    let elif_specials = [
                        &[HashMap::from([(
                            "condition".to_string(),
                            elif_condition_value.clone(),
                        )])],
                        &specials[..],
                    ]
                    .concat();

                    if elif_condition_value.as_bool() {
                        return Ok(
                            match elif_then.children.iter().try_fold(Value::Null, |_, child| {
                                interpret(child, depth + 1, variables, &elif_specials, functions)
                            }) {
                                Ok(val) => val,
                                Err(err) => match err.downcast::<BlockControl>() {
                                    Ok(BlockControl::Break(val)) => val,
                                    Ok(BlockControl::Continue) => {
                                        return Err(BlockControl::Continue.into());
                                    }
                                    Err(err) => return Err(err),
                                },
                            },
                        );
                    }
                }

                if let Some(else_block) = else_block {
                    match else_block
                        .children
                        .iter()
                        .try_fold(Value::Null, |_, child| {
                            interpret(child, depth + 1, variables, &specials, functions)
                        }) {
                        Ok(val) => val,
                        Err(err) => match err.downcast::<BlockControl>() {
                            Ok(BlockControl::Break(val)) => val,
                            Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),
                            Err(err) => return Err(err),
                        },
                    }
                } else {
                    Value::Null
                }
            }
        }

        "loop" => {
            let start = element
                .attributes
                .get("start")
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);

            let end = element
                .attributes
                .get("end")
                .and_then(|s| s.parse::<i64>().ok());

            let mut iteration = start;

            let mut specials = [
                &[HashMap::from([(
                    "iteration".to_string(),
                    Value::Int(iteration),
                )])],
                specials,
            ]
            .concat();

            'outer: loop {
                specials[0].insert("iteration".to_string(), Value::Int(iteration));

                if let Some(end) = end
                    && iteration >= end
                {
                    break 'outer Ok(Value::Null);
                }

                for child in &element.children {
                    if let Err(err) = interpret(child, depth + 1, variables, &specials, functions) {
                        match err.downcast::<BlockControl>() {
                            Err(e) => break 'outer Err(e),
                            Ok(BlockControl::Break(value)) => break 'outer Ok(value),
                            Ok(BlockControl::Continue) => continue 'outer,
                        }
                    }
                }

                iteration += 1;
            }?
        }

        "function" => {
            let name = element
                .attributes
                .get("name")
                .wrap_err("Expected the `name` attribute in <function> element")?
                .clone();

            ensure!(
                !name.is_empty(),
                "Function name cannot be empty in <function> element"
            );

            functions.insert(name, element.children.clone());

            Value::Null
        }

        "call" => {
            let name = element
                .attributes
                .get("name")
                .wrap_err("Expected the `name` attribute in <call> element")?
                .clone();

            let func = functions
                .get(&name)
                .cloned()
                .wrap_err(format!("Function `{name}` not found"))?;

            let children = element
                .children
                .iter()
                .map(|child| interpret(child, depth + 1, variables, specials, functions))
                .collect::<Result<Vec<Value>>>()?;

            let child_count = children.len();

            let children_specials = children
                .into_iter()
                .enumerate()
                .map(|(i, value)| (format!("child:{i}"), value))
                .chain(std::iter::once((
                    String::from("child_count"),
                    Value::Int(child_count as i64),
                )))
                .collect::<HashMap<_, _>>();

            let attrs = element
                .attributes
                .iter()
                .map(|(k, v)| (k.clone(), Value::from(v.as_str())))
                .collect::<HashMap<_, _>>();

            let specials = [&[attrs, children_specials], specials].concat();

            let mut variables = variables.clone();

            match func.into_iter().try_fold(Value::Null, |_, child| {
                interpret(&child, depth + 1, &mut variables, &specials, functions)
            }) {
                Ok(val) => val,
                Err(err) => match err.downcast::<BlockControl>() {
                    Ok(BlockControl::Break(val)) => val,
                    Ok(BlockControl::Continue) => return Err(BlockControl::Continue.into()),
                    Err(err) => return Err(err),
                },
            }
        }

        _ => bail!("Unknown element: {}", element.name),
    })
}
