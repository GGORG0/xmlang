use std::{env::args, path::Path};

use color_eyre::Result;

use crate::{element::Element, parser::parse};

mod element;
mod parser;
mod value;

fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: replace this with clap once we need more complex argument parsing
    let path = args()
        .nth(1)
        .expect("Usage: cargo run -- <path_to_xml_file>");

    let path = Path::new(&path);

    let tree = parse(path)?;
    print_tree(&tree, 0);

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
