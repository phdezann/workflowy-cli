use core::fmt;
use std::error::Error;

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

use conf::*;

use crate::conf;
use crate::error::{GenError, GenResult};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    pub project_tree_data: ProjectTreeData,
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTreeData {
    pub client_id: String,
    pub main_project_tree_info: MainProjectTreeInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MainProjectTreeInfo {
    pub root_project_children: Vec<Point>,
    pub initial_most_recent_operation_transaction_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "RootProjectChild", rename_all = "camelCase")]
pub struct Point {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "nm", default)]
    pub content: String,
    #[serde(rename = "no", default)]
    pub note: Option<String>,
    #[serde(rename = "cp", default)]
    pub complete: Option<u32>,
    #[serde(rename = "ch", default)]
    pub children: Option<Vec<Point>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Feature {
    codename: String,
    name: String,
    description: String,
}

#[derive(Debug)]
pub struct WorkflowyError {
    pub msg: String,
}

impl fmt::Display for WorkflowyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for WorkflowyError {}

pub fn get_tree(conf: Conf) -> GenResult<Tree> {
    let (status_code, json) = make_http_call(conf.session_id)?;
    match (status_code, json) {
        (StatusCode::OK, json) => Ok(deserialize_json(json)?),
        (_, _) => Err(GenError::from(WorkflowyError {
            msg: format!("Response code was {} instead of 200", status_code.as_u16()),
        })),
    }
}

fn make_http_call(session_id: String) -> GenResult<(StatusCode, String)> {
    let mut cookie = "sessionid=".to_string();
    cookie.push_str(&session_id);

    let request = Client::new()
        .get("https://workflowy.com/get_initialization_data")
        .header("accept", "application/json")
        .header("cookie", cookie)
        .query(&[("client_version", "21")]);

    let response = request.send()?;
    let status = response.status();
    let bytes = response.bytes()?.to_vec();
    let json = String::from_utf8_lossy(&bytes);
    Ok((status, json.to_string()))
}

fn deserialize_json(json: String) -> Result<Tree, serde_json::Error> {
    serde_json::from_str(&json)
}
