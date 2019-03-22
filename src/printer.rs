use crate::types::{BlockType, DocItem};

pub fn render(result: &[DocItem]) {
    for item in result.iter().filter(|i| i.category == BlockType::Comment) {
        print_title_block(&item.description);
    }
    print_resources(result, "Resources", BlockType::Resource);
    print_interface(result, "Inputs", BlockType::Variable);
    print_interface(result, "Outputs", BlockType::Output);
}

fn print_title_block(description: &[String]) {
    let title = &description.first().unwrap()["Title: ".len()..];
    println!("# {}\n", title);
    for line in description.iter().skip(1) {
        println!("{}", line);
    }
}

fn print_resources(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result
        .iter()
        .filter(|i| i.category == variant && i.description.len() > 0)
        .enumerate()
    {
        if index == 0 {
            println!("\n## {}\n", name);
        }
        if item.description.len() > 0 || variant != BlockType::Resource {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        }
    }
}

fn print_interface(result: &[DocItem], name: &str, variant: BlockType) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {}\n", name);
        }
        if item.description.len() > 0 {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        } else {
            println!("* `{}`", item.name);
        }
    }
}
