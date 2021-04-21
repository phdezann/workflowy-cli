extern crate clap;
extern crate home;
extern crate regex;
extern crate serde_json;

use std::process;

use cli::*;

use crate::cli::CliResult::{CliAuth, CliExport};

mod cli;
mod conf;
mod error;
mod printer;
mod serializer;
mod workflowy;

fn main() {
    match cli::get() {
        Some(CliAuth(_)) => {
            if let Err(err) = conf::save_session_id() {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
        Some(CliExport(export_args)) => {
            if let Err(err) = run(export_args) {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
        _ => (),
    }
}

fn run(export_args: Export) -> error::GenResult<()> {
    let conf = conf::read_conf()?;
    let tree = workflowy::get_tree(conf)?;
    let output = serializer::print(&export_args, &tree)?;
    printer::print(&export_args, output)
}
