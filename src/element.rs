use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}
