use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::types::{BlockType, Directive, DocItem};

pub fn parse_hcl(filename: &str) -> std::io::Result<Vec<DocItem>> {
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
        BlockType::Resource => {
            result.category = BlockType::Resource;
            result.name = parse_resource(&line);
        }
        BlockType::Output => {
            result.category = BlockType::Output;
            result.name = String::from(parse_output(&line));
        }
        BlockType::Variable => {
            result.category = BlockType::Variable;
            result.name = String::from(parse_variable(&line));
        }
        BlockType::Comment => {
            if line.trim_start_matches('#').trim().len() > 0 {
                result.category = BlockType::Comment;
                result.description.push(String::from(parse_comment(&line)));
            }
        }
        BlockType::None => {
            if (line.starts_with('}') && result.category != BlockType::None)
                || (line.trim().len() == 0 && result.category == BlockType::Comment)
            {
                return (result, Directive::Stop);
            }
            if result.category == BlockType::Variable {
                if let Some(description) = parse_variable_description(&line) {
                    result.description.push(String::from(description));
                }
            }
        }
    };
    (result, Directive::Continue)
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

fn parse_resource(line: &str) -> String {
    line.split_whitespace()
        .skip(1)
        .take(2)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()
        .join(".")
}

fn parse_variable(line: &str) -> &str {
    line.split_whitespace()
        .skip(1)
        .take(1)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()[0]
}

fn parse_variable_description(line: &str) -> Option<&str> {
    let start = line.find('"')?;
    let substring = line.get(start..)?;
    Some(substring.trim_matches('"'))
}

fn parse_output(line: &str) -> &str {
    line.split_whitespace()
        .skip(1)
        .take(1)
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()[0]
}

fn parse_comment(line: &str) -> &str {
    line.trim_start_matches('#').trim()
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
        assert_eq!(parse_output(line), "foo");
    }

    #[test]
    fn test_parse_variable() {
        let line = r#"variable "foo" {"#;
        assert_eq!(parse_variable(line), "foo");
    }

    #[test]
    fn test_parse_variable_description() {
        let line = r#"  description = "foo bar""#;
        assert_eq!(parse_variable_description(line), Some("foo bar"));
    }

    #[test]
    fn test_parse_comment() {
        let line = r#"# foo bar"#;
        assert_eq!(parse_comment(line), "foo bar");
    }

    #[test]
    fn test_parse_comment2() {
        let line = r#"#foo bar"#;
        assert_eq!(parse_comment(line), "foo bar");
    }
}
