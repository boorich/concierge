use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fmt::format};

use crate::streams::Stream;

#[derive(Debug, Serialize, Clone)]
pub struct Program {
    pub name: String,
    pub members: Vec<Stream>,
}

pub trait Actions {
    fn new(name: String, members: Vec<Stream>) -> Program;
    fn status(&self) -> String;
}

impl Actions for Program {
    fn new(name: String, members: Vec<Stream>) -> Program {
        Program {
            name: name,
            members: Vec::new(),
        }
    }
    fn status(&self) -> String {
        format!("{}", self.name)
    }
}
