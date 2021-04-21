extern crate home;
extern crate regex;
extern crate serde_json;

use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::{App, Arg};

use crate::cli::CliResult::{CliAuth, CliExport};
use crate::cli::Format::AnkiDict;

#[derive(Debug)]
pub struct CliError {
    pub msg: String,
}

impl Display for CliError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.msg)?;
        Ok(())
    }
}

impl Error for CliError {}

#[derive(Debug)]
pub enum Format {
    AnkiDict,
}

impl FromStr for Format {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, CliError> {
        if s.eq("anki-dict") {
            Ok(Format::AnkiDict)
        } else {
            Err(CliError {
                msg: format!("Can't parse format {:?}", s),
            })
        }
    }
}

pub struct Auth {}

pub struct Export {
    pub prefix: Vec<String>,
    pub append: Vec<String>,
    pub format: Format,
    pub output: String,
    pub root: String,
}

pub enum CliResult {
    CliAuth(Auth),
    CliExport(Export),
}

pub fn get() -> Option<CliResult> {
    let matches = App::new("Workflowy cli")
        .version("1.0")
        .subcommand(App::new("auth").about("register your http cookie to allow requests"))
        .subcommand(
            App::new("export")
                .about("export bullet point to csv")
                .arg(
                    Arg::new("format")
                        .short('f')
                        .possible_values(&["anki-dict"])
                        .about("Controls the output columns."),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .value_name("FILE")
                        .about("Sets the output file. Use '-' for standard output."),
                )
                .arg(
                    Arg::new("append")
                        .short('a')
                        .long("append")
                        .value_name("append")
                        .multiple(true)
                        .takes_value(true)
                        .about("Adds a columns with a fixed value."),
                )
                .arg(
                    Arg::new("prefix")
                        .short('p')
                        .long("prefix")
                        .value_name("prefix")
                        .multiple(true)
                        .takes_value(true)
                        .about("Adds a prefix selector, the suffix will be extracted as column"),
                )
                .arg(
                    Arg::new("root")
                        .about("Sets the root")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let auth = matches.subcommand_matches("auth").map(|_| CliAuth(Auth {}));
    let export = matches.subcommand_matches("export").map(|matches| {
        let prefix: Option<Vec<String>> = matches
            .values_of("prefix")
            .map(|prefixes| prefixes.map(|prefixes| prefixes.to_string()).collect());
        let append: Option<Vec<String>> = matches
            .values_of("append")
            .map(|append| append.map(|append| append.to_string()).collect());
        let format: Format = match matches.value_of("format").unwrap() {
            "anki-dict" => AnkiDict,
            _ => unreachable!(),
        };
        let output = matches.value_of("output").unwrap().to_string();
        let root = matches.value_of("root").unwrap().to_string();
        CliExport(Export {
            prefix: prefix.unwrap_or(vec![]),
            append: append.unwrap_or(vec![]),
            format,
            output,
            root,
        })
    });

    auth.or(export)
}
