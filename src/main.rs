use std::{collections::HashMap, env::args, path::Path};

use miette::Result;

use crate::{element::Element, interpreter::interpret, parser::parse};

mod element;
mod interpreter;
mod parser;
mod value;

fn main() -> Result<()> {
    miette::set_panic_hook();

    // TODO: replace this with clap once we need more complex argument parsing
    let path = args()
        .nth(1)
        .expect("Usage: cargo run -- <path_to_xml_file>");

    let path = Path::new(&path);

    let tree = parse(path)?;
    // print_tree(&tree, 0);

    let mut variables = HashMap::new();

    interpret(&tree, 0, &mut variables, &[])?;

    Ok(())
}

fn print_tree(element: &Element, indent: usize) {
    let indent_str = " ".repeat(indent);

    let attrs_str = if element.attributes.is_empty() {
        String::new()
    } else {
        let attrs: Vec<String> = element
            .attributes
            .iter()
            .map(|(k, v)| format!("{k}=\"{v}\""))
            .collect();
        format!(
            "{}{}",
            if element.name.is_empty() { "" } else { " " },
            attrs.join(" ")
        )
    };

    println!(
        "{}{}{}{}",
        indent_str,
        element
            .namespace
            .as_ref()
            .map(|x| [x, " -> "].concat())
            .unwrap_or_default(),
        element.name,
        attrs_str
    );

    for child in &element.children {
        print_tree(child, indent + 2);
    }
}
