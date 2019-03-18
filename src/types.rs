use std::fmt;

#[derive(Debug)]
pub struct DocItem {
    pub category: Option<BlockVariant>,
    pub name: String,
    pub description: Vec<String>,
}

impl DocItem {
    pub fn new() -> DocItem {
        DocItem {
            category: None,
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

#[derive(Debug)]
pub enum BlockVariant {
    Comment,
    Resource,
    Output,
    Variable,
}

// impl fmt::Display for BlockVariant {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let diff = match self.diff {
//             '.' => "".to_owned(),
//             _ => format!("{} ", self.diff),
//         };
//         write!(f, "{}{}{}", " ".repeat(self.depth * 2), diff, self.contents)
//     }
// }

pub enum Directive {
    Continue,
    Stop,
}
