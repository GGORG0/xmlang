use std::path::Path;

use color_eyre::Result;

use crate::parser::{Element, parse};

mod parser;

fn main() -> Result<()> {
    color_eyre::install()?;

    let test_file = Path::new("examples/test.xml");

    let tree = parse(test_file)?;
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
