use core::fmt;
use std::error::Error;

use regex::{escape, Regex};

use cli::*;
use workflowy::*;

use crate::cli;
use crate::error::GenResult;
use crate::workflowy;

#[derive(Debug)]
pub struct SerializerError {
    pub msg: String,
}

impl fmt::Display for SerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SerializerError {}

pub fn print(export: &Export, dictionary: &Tree) -> Result<String, SerializerError> {
    let point_id = extract_root_id(&export.root).ok_or(SerializerError {
        msg: format!("Cannot extract root id from {:?}", &export.root),
    })?;

    let root = traverse_trees(
        &dictionary
            .project_tree_data
            .main_project_tree_info
            .root_project_children,
        &point_id,
    )
    .ok_or(SerializerError {
        msg: format!("Cannot find root from id from {:?}", point_id),
    })?;

    let csv = export_to_csv(&export, root);

    Ok(csv)
}

fn export_to_csv(export: &Export, dictionary: &Point) -> String {
    let separator = "\t";

    let output = dictionary.children.as_ref().map(|words| {
        words
            .iter()
            .flat_map(|word| {
                word.children.as_ref().map(|attributes| {
                    let word = word.content.clone().trim().to_string();
                    let definitions: Vec<String> = attributes
                        .iter()
                        .map(|attribute| &attribute.content)
                        .flat_map(|content| {
                            export
                                .prefix
                                .iter()
                                .flat_map(move |prefix| find_remove_prefix(content.clone(), prefix))
                                .filter_map(|e| e)
                                .map(strip_prefix)
                        })
                        .map(trim)
                        .collect();
                    (word, definitions)
                })
            })
            .fold("".to_string(), |mut acc, (word, definitions)| {
                acc.push_str(&word);
                acc.push_str(separator);
                definitions.iter().for_each(|definition| {
                    acc.push_str(&definition);
                    acc.push_str(separator);
                });
                export.append.iter().for_each(|append| {
                    acc.push_str(&append);
                    acc.push_str(separator);
                });
                if acc.ends_with(separator) {
                    acc.pop();
                }
                acc.push_str("\n");
                acc
            })
    });

    output.unwrap_or("".to_string())
}

fn traverse_trees<'a>(trees: &'a Vec<Point>, point_id: &'a String) -> Option<&'a Point> {
    fn traverse_tree<'a>(tree: &'a Point, point_id: &'a String) -> Option<&'a Point> {
        if tree.id.ends_with(point_id) {
            Some(tree)
        } else {
            tree.children.as_ref().and_then(|points| {
                points
                    .into_iter()
                    .map(|p| traverse_tree(p, point_id))
                    .filter_map(|p| p)
                    .collect::<Vec<&Point>>()
                    .first()
                    .map(|p| *p)
            })
        }
    }

    trees
        .iter()
        .map(|p| traverse_tree(p, point_id))
        .filter_map(|p| p)
        .collect::<Vec<&Point>>()
        .first()
        .map(|p| *p)
}

fn find_remove_prefix(content: String, prefix: &String) -> GenResult<Option<String>> {
    let payload = Regex::new(format!(r"(?i)(<.*?>)*{}(?P<name>.*)", escape(prefix)).as_str())?
        .captures(content.as_str())
        .map(|cap| Some(cap.name("name").unwrap().as_str().to_string()))
        .unwrap_or(None);
    Ok(payload)
}

fn strip_prefix(content: String) -> String {
    Regex::new(r"(\s|:|(</.*?>))*(?P<name>.*)")
        .unwrap()
        .captures(&content)
        .map(|cap| cap.name("name").unwrap().as_str().to_string())
        .unwrap_or(content)
}

fn trim(content_no_tag: String) -> String {
    content_no_tag.trim().to_string()
}

fn extract_root_id(raw_root_id: &String) -> Option<String> {
    raw_root_id.split("/").last().map(|id| id.to_string())
}

#[test]
fn test_find_remove_prefix() {
    assert_eq!(
        find_remove_prefix(
            "example(s): this is an example".to_string(),
            &"example(s)".to_string(),
        )
        .unwrap(),
        Some(": this is an example".to_string())
    );
}

#[test]
fn test_extract_root_id_from_url() {
    assert_eq!(
        extract_root_id(&"https://workflowy.com/#/6af512b586de".to_string()),
        Some("6af512b586de".to_string())
    );
}

#[test]
fn test_extract_root_id_from_id() {
    assert_eq!(
        extract_root_id(&"6af512b586de".to_string()),
        Some("6af512b586de".to_string())
    );
}
