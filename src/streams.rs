use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::format;

#[derive(Debug)]
struct Dependency {
    name: String,
    version: String,
    path: String,
    optional: bool,
    default_features: bool,
    features: Vec<String>,
    target: Option<String>,
    registry: Option<String>,
}
#[derive(Debug)]
struct Feature {
    name: String,
    dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Stream {
    pub name: String,
    version: String,
    authors: Vec<String>,
    license: String,
    repository: String,
    homepage: String,
    documentation: String,
    readme: String,
    keywords: Vec<String>,
    categories: Vec<String>,
    include: Vec<String>,
    exclude: Vec<String>,
}

pub trait Actions {
    fn new(name: String) -> Stream;
    fn status(&self) -> String;
}

impl Actions for Stream {
    fn new(name: String) -> Stream {
        Stream {
            name: name,
            version: String::from("0.1.0"),
            authors: Vec::new(),
            license: String::from(""),
            repository: String::from(""),
            homepage: String::from(""),
            documentation: String::from(""),
            readme: String::from(""),
            keywords: Vec::new(),
            categories: Vec::new(),
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
    fn status(&self) -> String {
        format!("{} {} {}", self.name, self.version, self.repository)
    }
}
