use std::fmt;

#[derive(Debug)]
pub struct DocItem {
    pub category: BlockType,
    pub name: String,
    pub description: Vec<String>,
}

impl DocItem {
    pub fn new() -> DocItem {
        DocItem {
            category: BlockType::None,
            name: String::new(),
            description: vec![],
        }
    }
}

impl fmt::Display for DocItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.len() > 0 {
            write!(f, "`{}`: {}", self.name, self.description.join(" "))
        } else {
            write!(f, "{}", self.description.join(" "))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Comment,
    Resource,
    Output,
    Variable,
    None,
}
