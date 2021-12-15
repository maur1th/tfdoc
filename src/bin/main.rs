//! The main binary file responsible for parsing the Terraform files and outputting the Markdown code.
//!
//! Usage: `tfdoc [-t] PATH`
//!
//! If PATH is omitted, the current directory is used.
//!
//! Use the `-t` parameter to output the documentation in table format rather than list format.

use std::env;
use std::io;
use std::path::Path;

extern crate tfdoc;
use tfdoc::{parser, printer, types, util};

/// The main function responsible for actually carrying out the work.
fn run_app() -> io::Result<()> {
    // Look for the path or just use the current directory if none is given
    let use_tables: bool;
    let path_arg: String;

    // If the -t parameter has been supplied, output the contents as tables
    if env::args().len() > 1 && env::args().nth(1).unwrap() == *"-t" {
        use_tables = true;
        path_arg = env::args().nth(2).unwrap_or_else(|| String::from("./"));
    } else {
        use_tables = false;
        path_arg = env::args().nth(1).unwrap_or_else(|| String::from("./"));
    }

    // Find just the Terraform files
    let tf_files = util::list_tf_files(Path::new(&path_arg))?;

    // Parse the files found and put them into a list
    let mut result: Vec<types::DocItem> = vec![];
    for tf_file in &tf_files {
        result.append(&mut parser::parse_hcl(tf_file.to_path_buf())?);
    }

    // Output the resulting markdown
    printer::render(&result, use_tables);
    printer::print_files(&tf_files, use_tables);

    // Return safely
    Ok(())
}

/// Calls `run_app` and exits with error code `0` if successful. Otherwise prints an error message to `stderr` and exits with error code `1`.
fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}
