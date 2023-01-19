//! Outputs the resulting information in Markdown format, using lists or tables, depending on preference.

use std::path::PathBuf;

use crate::types::{BlockType, DocItem};

/// Renders the results to a markdown file
pub fn render(result: &[DocItem], as_table: bool) {
    // Print the H1 Title: block
    for item in result.iter().filter(|i| i.category == BlockType::Comment) {
        print_title_block(&item.description);
    }

    // Print the H2 Blocks
    if as_table {
        print_resources_table(result, "Resource", BlockType::Resource);
        print_interface_table(result, "Input", BlockType::Variable);
        print_interface_table(result, "Output", BlockType::Output);
    } else {
        print_resources(result, "Resource", BlockType::Resource);
        print_interface(result, "Inputs", BlockType::Variable);
        print_interface(result, "Outputs", BlockType::Output);
    }
}

/// Creates the H1 title block
fn print_title_block(description: &[String]) {
    let blank_string = String::new();
    let title = &description.first().unwrap_or(&blank_string)["Title: ".len()..];
    println!("# {title}\n");
    for line in description.iter().skip(1) {
        println!("{line}");
    }
}

/// Outputs the `resource` items as a list
fn print_resources(result: &[DocItem], name: &str, variant: BlockType) {
    log::debug!("print_resources::result = {:?}", result);
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {name}s\n");
        }
        if variant == BlockType::Resource {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the interfaces (ie. the `variable` and `output` sections) as a list
fn print_interface(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {name}s\n");
        }
        if item.description.is_empty() {
            println!("* `{}`", item.name);
        } else {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the `resource` items as a table
fn print_resources_table(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {name}s");
            println!("\n|{name}|Description|\n|-----|---------|");
        }
        if variant == BlockType::Resource {
            println!("|`{}`|{}|", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the interfaces (ie. the `variable` and `output` sections) as a table
fn print_interface_table(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {name}s");
            println!("\n|{name}|Description|\n|-----|---------|");
        }
        if item.description.is_empty() {
            println!("|`{}`||", item.name);
        } else {
            println!("|`{}`|{}|", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the file list either as a table or a list, depending on what's chosen
pub fn print_files(files: &[PathBuf], table: bool) {
    println!("\n## Files\n");
    if table {
        println!("|File Name|Description|\n|-----|---------|");
    }
    for file in files {
        if table {
            println!("|`{}`||", &file.to_str().unwrap_or("Unknown"));
        } else {
            println!("* `{}`", &file.to_str().unwrap_or("Unknown"));
        }
    }
}
