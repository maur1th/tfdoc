//! Implements the types used to identify the various sections of the Terraform files.

use std::fmt;

/// Holds the various entities to be exported to the documentation
#[derive(Debug)]
pub struct DocItem {
    /// The type of entity, ie. `comment`, `resource`, `output`, `variable`.
    pub category: BlockType,

    /// The name of the entity
    pub name: String,

    /// The `#` comments and/or `description` fields associated with the entity
    pub description: Vec<String>,
}

impl DocItem {
    /// Creates a new empty `DocItem` entity
    #[must_use] pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DocItem {
    /// Creates a default `DocItem`
    fn default() -> Self {
        Self {
            category: BlockType::None,
            name: String::new(),
            description: vec![],
        }
    }
}

impl fmt::Display for DocItem {
    /// Formats a `DocItem` for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.name.is_empty() {
            write!(f, "`{}`: {}", self.name, self.description.join(" "))
        } else {
            write!(f, "{}", self.description.join(" "))
        }
    }
}

/// Enumerates the types of blocks recognized.
#[derive(Debug, PartialEq)]
pub enum BlockType {
    Comment,
    Resource,
    Output,
    Variable,
    None,
}
