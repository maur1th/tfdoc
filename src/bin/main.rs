use std::env;
use std::io;
use std::path::Path;

extern crate tfdoc;
use tfdoc::{parser, printer, types, util};

fn run_app() -> io::Result<()> {
    let path_arg = env::args().nth(1).unwrap_or(String::from("./"));
    let path = Path::new(&path_arg);
    let mut result: Vec<types::DocItem> = vec![];
    let files = util::list_files(path)?;
    for file_path in files {
        result.append(&mut parser::parse_hcl(file_path)?);
    }
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
