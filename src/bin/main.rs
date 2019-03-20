use std::io;

extern crate tfdoc;
use tfdoc::{parser, printer};

fn run_app() -> io::Result<()> {
    let result = parser::parse_hcl("variables.tf")?;
    printer::render(&result);
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
