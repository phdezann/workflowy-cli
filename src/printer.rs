use std::fs::File;
use std::io::Write;

use cli::*;
use error::GenResult;

use crate::cli;
use crate::error;

pub fn print(conf: &Export, output: String) -> GenResult<()> {
    if conf.output.eq("-") {
        println!("{}", output);
        Ok(())
    } else {
        let mut file = File::create(&conf.output)?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }
}
