use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::types::{BlockVariant, Directive, DocItem};

pub fn parse_hcl(filename: &str) -> std::io::Result<Vec<DocItem>> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut result = vec![DocItem::new()];
    for line in buf_reader.lines() {
        let state = parse_line(line?, result.pop().unwrap());
        result.push(state.0);
        if let Directive::Stop = state.1 {
            result.push(DocItem::new());
        }
    }
    result.pop(); // Remove the last DocItem from the collection
    Ok(result)
}

fn parse_line(line: String, mut result: DocItem) -> (DocItem, Directive) {
    match get_line_variant(&line) {
        Some(BlockVariant::Resource) => {
            result.category = Some(BlockVariant::Resource);
            result.name = parse_resource(&line);
        }
        Some(BlockVariant::Output) => {
            result.category = Some(BlockVariant::Output);
            result.name = String::from(parse_output(&line));
        }
        Some(BlockVariant::Variable) => {
            result.category = Some(BlockVariant::Variable);
            result.name = String::from(parse_variable(&line));
        }
        Some(BlockVariant::Comment) => {
            if line.trim_start_matches('#').trim().len() > 0 {
                result.category = Some(BlockVariant::Comment);
                result.description.push(String::from(parse_comment(&line)));
            }
        }
        None => {
            if line.starts_with('}') && result.category.is_some() {
                return (result, Directive::Stop);
            }
            if line.trim().len() == 0 {
                if let Some(BlockVariant::Comment) = result.category {
                    return (result, Directive::Stop);
                }
            }
            if let Some(BlockVariant::Variable) = result.category {
                if let Some(description) = parse_variable_description(&line) {
                    result.description.push(String::from(description));
                }
            }
        }
    };
    (result, Directive::Continue)
}

fn get_line_variant(line: &str) -> Option<BlockVariant> {
    let variants = vec![
        ("resource ", BlockVariant::Resource),
        ("variable ", BlockVariant::Variable),
        ("output ", BlockVariant::Output),
        ("#", BlockVariant::Comment),
    ];
    for variant in variants {
        if line.starts_with(variant.0) {
            return Some(variant.1);
        }
    }
    None
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
            Some(BlockVariant::Resource) => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_output() {
        let line = r#"output "foo" {"#;
        match get_line_variant(line) {
            Some(BlockVariant::Output) => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_variable() {
        let line = r#"variable "foo" {"#;
        match get_line_variant(line) {
            Some(BlockVariant::Variable) => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_comment() {
        let line = r#"# foo"#;
        match get_line_variant(line) {
            Some(BlockVariant::Comment) => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_comment2() {
        let line = r#"#foo"#;
        match get_line_variant(line) {
            Some(BlockVariant::Comment) => {}
            _ => panic!("Type error!"),
        }
    }

    #[test]
    fn get_line_variant_none() {
        let line = r#"  foo"#;
        match get_line_variant(line) {
            None => {}
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
