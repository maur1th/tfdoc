//! Outputs the resulting information in Markdown format, using lists or tables, depending on preference.

use crate::types::{BlockType, DocItem};

/// Renders the results to a markdown file
pub fn render(result: &[DocItem], as_table: bool) {
    for item in result.iter().filter(|i| i.category == BlockType::Comment) {
        print_title_block(&item.description);
    }
    if as_table {
        print_resources_table(result, "Resource", BlockType::Resource);
        print_interface_table(result, "Input", BlockType::Variable);
        print_interface_table(result, "Output", BlockType::Output);
    } else {
        print_resources(result, "Resources", BlockType::Resource);
        print_interface(result, "Inputs", BlockType::Variable);
        print_interface(result, "Outputs", BlockType::Output);
    }
}

/// Creates the H1 title block
fn print_title_block(description: &[String]) {
    let title = &description.first().unwrap()["Title: ".len()..];
    println!("# {}\n", title);
    for line in description.iter().skip(1) {
        println!("{}", line);
    }
}

/// Outputs the `resource` items as a list
fn print_resources(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result
        .iter()
        .filter(|i| i.category == variant && !i.description.is_empty())
        .enumerate()
    {
        if index == 0 {
            println!("\n## {}\n", name);
        }
        if !item.description.is_empty() || variant != BlockType::Resource {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the interfaces (ie. the `variable` and `output` sections) as a list
fn print_interface(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {}\n", name);
        }
        if !item.description.is_empty() {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        } else {
            println!("* `{}`", item.name);
        }
    }
}

/// Outputs the `resource` items as a table
fn print_resources_table(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result
        .iter()
        .filter(|i| i.category == variant && !i.description.is_empty())
        .enumerate()
    {
        if index == 0 {
            println!("\n## {}s", name);
            println!("\n|{}|Description|\n|-----|---------|", name);
        }
        if !item.description.is_empty() || variant != BlockType::Resource {
            println!("|`{}`|{}|", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the interfaces (ie. the `variable` and `output` sections)
fn print_interface_table(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {}s", name);
            println!("\n|{}|Description|\n|-----|---------|", name);
        }
        if !item.description.is_empty() {
            println!("|`{}`|{}|", item.name, item.description.join(" "));
        } else {
            println!("|`{}`||", item.name);
        }
    }
}
