use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::{format, Display};

use crate::{programs::Program, streams::Person};

#[derive(Debug, Serialize, Clone)]
pub struct Forum {
    pub name: String,
    pub members: Vec<Program>,
}

pub trait Actions {
    fn new(name: String, members: Vec<Person>) -> Forum;
    fn calculate_slio_effects(&self);
    fn calculate_synergies(&self);
}

impl Actions for Forum {
    fn new(name: String, members: Vec<Person>) -> Forum {
        Forum {
            name: name,
            members: Vec::new(),
        }
    }
    fn calculate_slio_effects(&self) {
        //ToDo: Calculate silo effects between programs
        //for each program here: call program:: calc_silo
        //Then calculate synergies between programs
    }

    fn calculate_synergies(&self) {
        //ToDo: Figure out here how to calculate synergies between streams within any program
    }
}impl serde::Serialize for Forum {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Forum", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("members", &self.members)?;
        state.end()
    }
}
