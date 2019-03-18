use std::io;

extern crate tfdoc;
use tfdoc::parser::parse_hcl;

fn run_app() -> io::Result<()> {
    let result = parse_hcl("variables.tf")?;
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}
