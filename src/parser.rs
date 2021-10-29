//! Contains the functionality for parsing Terraform files and returning the
//! relevant comments and descriptions for the various sections.

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use crate::types::{BlockType, DocItem};

#[derive(PartialEq)]
enum Directive {
    Continue,
    Stop,
}

/// Read a file and return a parsed list of DocItems
pub fn parse_hcl(filename: PathBuf) -> std::io::Result<Vec<DocItem>> {
    let mut result = vec![DocItem::new()];

    // Read the lines in the file and parse
    for line in BufReader::new(File::open(filename)?).lines() {
        let state = parse_line(line?, result.pop().unwrap());
        result.push(state.0);
        if state.1 == Directive::Stop {
            result.push(DocItem::new());
        }
    }

    result.pop(); // Remove the last DocItem from the collection since it's empty

    // Parse the results to look for the Title:
    result = result
        .into_iter()
        .filter(|i| {
            if i.category == BlockType::Comment {
                if let Some(line) = i.description.first() {
                    return line.starts_with("Title: ");
                }
            }
            true
        })
        .collect();

    // Return the result
    Ok(result)
}

/// Parse an individual line and return the type of DocItem found and what to do next
fn parse_line(line: String, mut result: DocItem) -> (DocItem, Directive) {
    match get_line_variant(&line) {
        // Check what type of line it is
        BlockType::Resource => parse_regular(line, result, BlockType::Resource, &parse_resource),
        BlockType::Output => parse_regular(line, result, BlockType::Output, &parse_interface),
        BlockType::Variable => parse_regular(line, result, BlockType::Variable, &parse_interface),
        BlockType::Comment => (parse_comment(line, result), Directive::Continue),
        BlockType::None => {
            // Determine whether to stop parsing this block
            if (line.starts_with('}') && result.category != BlockType::None)
                || (line.trim().is_empty() && result.category == BlockType::Comment)
            {
                return (result, Directive::Stop);
            }
            // Parse description if relevant
            if (result.category == BlockType::Variable || result.category == BlockType::Output)
                && line.trim().starts_with("description")
            {
                if let Some(description) = parse_description(&line) {
                    result.description.push(String::from(description));
                }
            }
            (result, Directive::Continue)
        }
    }
}

/// See if a line starts with any of the known variants and assign the correponding BlockType
fn get_line_variant(line: &str) -> BlockType {
    let variants = vec![
        ("resource ", BlockType::Resource),
        ("variable ", BlockType::Variable),
        ("output ", BlockType::Output),
        ("#", BlockType::Comment),
        ("//", BlockType::Comment),
    ];
    for variant in variants {
        if line.starts_with(variant.0) {
            return variant.1;
        }
    }
    BlockType::None
}

/// Parse `regular` or `interface` (ie. `resource` vs `variable` or `output`) blocks
fn parse_regular(
    line: String,
    mut result: DocItem,
    category: BlockType,
    parser_function: &dyn Fn(&str) -> String,
) -> (DocItem, Directive) {
    result.category = category;
    result.name = parser_function(&line);
    match line.trim().ends_with('}') {
        true => (result, Directive::Stop),
        false => (result, Directive::Continue),
    }
}

/// Parse a `resource` block
fn parse_resource(line: &str) -> String {
    line.split_whitespace()
        .skip(1)
        .take(2)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()
        .join(".")
}

/// Parse `variable` and `output` blocks
fn parse_interface(line: &str) -> String {
    let result = line
        .split_whitespace()
        .skip(1)
        .take(1)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()[0];
    String::from(result)
}

/// Parse `description` items
fn parse_description(line: &str) -> Option<&str> {
    let start = line.find('"')?;
    let substring = line.get(start..)?;
    Some(substring.trim_matches('"'))
}

/// Parse comment blocks
fn parse_comment(line: String, mut result: DocItem) -> DocItem {
    let parsed;

    if line.starts_with('#') {
        parsed = line.trim_start_matches('#').trim();
    } else {
        parsed = line.trim_start_matches("//").trim();
    }

    if !parsed.is_empty() {
        result.category = BlockType::Comment;
        result.description.push(String::from(parsed));
    }
    result
}

//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_line_variant_resource() {
        let line = r#"resource "foo" "bar" {"#;
        match get_line_variant(line) {
            BlockType::Resource => {}
            _ => panic!("Type error! Expected Resource but found something else."),
        }
    }

    #[test]
    fn get_line_variant_output() {
        let line = r#"output "foo" {"#;
        match get_line_variant(line) {
            BlockType::Output => {}
            _ => panic!("Type error! Expected Output but found something else."),
        }
    }

    #[test]
    fn get_line_variant_variable() {
        let line = r#"variable "foo" {"#;
        match get_line_variant(line) {
            BlockType::Variable => {}
            _ => panic!("Type error! Expected Variable but found something else."),
        }
    }

    #[test]
    fn get_line_variant_comment() {
        let line = r#"# foo"#;
        match get_line_variant(line) {
            BlockType::Comment => {}
            _ => panic!("Type error! Expected Comment but found something else."),
        }
    }

    #[test]
    fn get_line_variant_comment2() {
        let line = r#"#foo"#;
        match get_line_variant(line) {
            BlockType::Comment => {}
            _ => panic!("Type error! Expected Comment but found something else."),
        }
    }

    #[test]
    fn get_line_variant_none() {
        let line = r#"  foo"#;
        match get_line_variant(line) {
            BlockType::None => {}
            _ => panic!("Type error! Expected None but found someething else."),
        }
    }

    #[test]
    fn test_parse_resource() {
        let line = r#"resource "foo" "bar" {"#;
        assert_eq!(parse_resource(line), "foo.bar".to_string());
    }

    #[test]
    fn test_parse_output() {
        let line = r#"output "foo" {"#;
        assert_eq!(parse_interface(line), "foo");
    }

    #[test]
    fn test_parse_variable() {
        let line = r#"variable "foo" {"#;
        assert_eq!(parse_interface(line), "foo");
    }

    #[test]
    fn test_parse_description() {
        let line = r#"  description = "foo bar""#;
        assert_eq!(parse_description(line), Some("foo bar"));
    }

    #[test]
    fn test_parse_comment() {
        let line = String::from(r#"# foo bar"#);
        let result = DocItem::new();
        assert_eq!(parse_comment(line, result).description[0], "foo bar");
    }

    #[test]
    fn test_parse_comment2() {
        let line = String::from(r#"#foo bar"#);
        let result = DocItem::new();
        assert_eq!(parse_comment(line, result).description[0], "foo bar");
    }
}
