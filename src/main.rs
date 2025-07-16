use std::{collections::HashMap, env::args, path::Path};

use miette::Result;

use crate::{interpreter::interpret, parser::parse};

mod element;
mod interpreter;
mod parser;
mod value;

fn main() -> Result<()> {
    miette::set_panic_hook();

    // TODO: replace this with clap once we need more complex argument parsing
    let path = args()
        .nth(1)
        .expect("Provide the path to the XML file as the first command line argument");

    let path = Path::new(&path);

    let tree = parse(path)?;

    let mut variables = HashMap::new();

    interpret(&tree, 0, &mut variables, &[])?;

    Ok(())
}
