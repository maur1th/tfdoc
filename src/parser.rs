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

pub fn parse_hcl(filename: PathBuf) -> std::io::Result<Vec<DocItem>> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut result = vec![DocItem::new()];
    for line in buf_reader.lines() {
        let state = parse_line(line?, result.pop().unwrap());
        result.push(state.0);
        if state.1 == Directive::Stop {
            result.push(DocItem::new());
        }
    }
    result.pop(); // Remove the last DocItem from the collection since it's empty
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
    Ok(result)
}

fn parse_line(line: String, mut result: DocItem) -> (DocItem, Directive) {
    match get_line_variant(&line) {
        BlockType::Resource => parse_regular(line, result, BlockType::Resource, &parse_resource),
        BlockType::Output => parse_regular(line, result, BlockType::Output, &parse_interface),
        BlockType::Variable => parse_regular(line, result, BlockType::Variable, &parse_interface),
        BlockType::Comment => (parse_comment(line, result), Directive::Continue),
        BlockType::None => {
            // Determine if it should stop parsing this block
            if (line.starts_with('}') && result.category != BlockType::None)
                || (line.trim().len() == 0 && result.category == BlockType::Comment)
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

fn get_line_variant(line: &str) -> BlockType {
    let variants = vec![
        ("resource ", BlockType::Resource),
        ("variable ", BlockType::Variable),
        ("output ", BlockType::Output),
        ("#", BlockType::Comment),
    ];
    for variant in variants {
        if line.starts_with(variant.0) {
            return variant.1;
        }
    }
    BlockType::None
}

fn parse_regular(
    line: String,
    mut result: DocItem,
    category: BlockType,
    parser_function: &Fn(&str) -> String,
) -> (DocItem, Directive) {
    result.category = category;
    result.name = parser_function(&line);
    match line.trim().ends_with('}') {
        true => (result, Directive::Stop),
        false => (result, Directive::Continue),
    }
}

fn parse_resource(line: &str) -> String {
    line.split_whitespace()
        .skip(1)
        .take(2)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()
        .join(".")
}

fn parse_interface(line: &str) -> String {
    // Parse variable and output blocks
    let result = line
        .split_whitespace()
        .skip(1)
        .take(1)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()[0];
    String::from(result)
}

fn parse_description(line: &str) -> Option<&str> {
    let start = line.find('"')?;
    let substring = line.get(start..)?;
    Some(substring.trim_matches('"'))
}

fn parse_comment(line: String, mut result: DocItem) -> DocItem {
    let parsed = line.trim_start_matches('#').trim();
    if parsed.len() > 0 {
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
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_output() {
        let line = r#"output "foo" {"#;
        match get_line_variant(line) {
            BlockType::Output => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_variable() {
        let line = r#"variable "foo" {"#;
        match get_line_variant(line) {
            BlockType::Variable => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_comment() {
        let line = r#"# foo"#;
        match get_line_variant(line) {
            BlockType::Comment => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_comment2() {
        let line = r#"#foo"#;
        match get_line_variant(line) {
            BlockType::Comment => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_none() {
        let line = r#"  foo"#;
        match get_line_variant(line) {
            BlockType::None => {}
            _ => panic!("Type error!"),
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
