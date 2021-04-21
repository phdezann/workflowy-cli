use core::fmt;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

use directories::ProjectDirs;
use serde::Deserialize;

use error::GenResult;

use crate::error;

#[derive(Debug, PartialEq, serde::Serialize, Deserialize)]
pub struct Conf {
    pub session_id: String,
}

#[derive(Debug)]
pub struct ConfError {
    pub msg: String,
}

impl fmt::Display for ConfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ConfError {}

pub fn read_conf() -> error::GenResult<Conf> {
    let config_file = get_configuration_path()?;
    let result = File::open(config_file).map_err(|_| ConfError {
        msg: "Cannot find sessionid, did you run 'workflowy auth'?".to_string(),
    })?;
    let conf: Conf = serde_yaml::from_reader(result)?;
    Ok(conf)
}

pub fn save_session_id() -> GenResult<()> {
    print!("sessionid=");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let session_id = buffer.trim_end().to_string();
    let conf = Conf { session_id };
    let conf_as_string = serde_yaml::to_string(&conf);
    let config_file = get_configuration_path()?;
    let config_dir = config_file.parent().unwrap();
    fs::create_dir_all(config_dir)?;
    let mut file = File::create(config_file)?;
    file.write_all(conf_as_string?.as_bytes())?;
    Ok(())
}

fn get_configuration_path() -> GenResult<PathBuf> {
    let config_dir = ProjectDirs::from("", "", "workflowy-cli")
        .ok_or(ConfError {
            msg: "Cannot access configuration directory".to_string(),
        })?
        .config_dir()
        .to_path_buf();
    let config_file = config_dir.join(Path::new("workflowy-cli.yml"));
    Ok(config_file)
}
