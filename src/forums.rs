use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fmt::format};

use crate::programs::Program;

#[derive(Debug, Serialize, Clone)]
pub struct Forum {
    pub name: String,
    pub members: Vec<Program>,
}

pub trait Actions {
    fn new(name: String, members: Vec<Program>) -> Forum;
    fn status(&self) -> String;
}

impl Actions for Forum {
    fn new(name: String, members: Vec<Program>) -> Forum {
        Forum {
            name: name,
            members: Vec::new(),
        }
    }
    fn status(&self) -> String {
        format!("{}", self.name)
    }
}
