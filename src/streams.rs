use chrono::prelude::*;
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{de, Result};
use std::fmt::format;

#[derive(Debug, Serialize, Clone)]
pub struct Person {
    name: String,
    skill: Vec<String>,
}

impl Person {
    pub fn new(name: String, skill: Vec<String>) -> Person {
        Person {
            name: name,
            skill: skill,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Dependency {
    deadline: chrono::DateTime<chrono::Utc>,
    budget: u32,
    expert: Person,
    code: Vec<String>,
    legal: Vec<String>, //optional field for legal documents
}

impl Dependency {
    fn new(
        deadline: chrono::DateTime<chrono::Utc>,
        budget: u32,
        expert: Person,
        code: Vec<String>,
        legal: Vec<String>,
    ) -> Dependency {
        Dependency {
            deadline: deadline,
            budget: budget,
            expert: expert,
            code: code,
            legal: legal,
        }
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Dependency {
            deadline: chrono::Utc::now(),
            budget: 0,
            expert: Person::new("".to_string(), Vec::new()),
            code: Vec::new(),
            legal: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Stream {
    pub name: String,                // name of the stream
    pub description: String,         // description of the stream
    pub stream_lead: Person,         // stream lead
    pub team: Vec<Person>,           // team members
    pub dependency: Vec<Dependency>, // dependencies
    pub repository: String,          // code repository
    pub readme: String,              // what is this stream about
    pub keywords: Vec<String>,       // keywords for tagging
    pub documentation: String,       // optional documentation
    pub homepage: String,            // optional homepage
}

pub trait Actions {
    fn new(
        name: String,
        description: String,
        stream_lead: Person,
        team: Vec<Person>,
        dependency: Vec<Dependency>,
        repository: String,
        readme: String,
        keywords: Vec<String>,
        documentation: String,
        homepage: String,
    ) -> Stream;
    fn status(&self);
    fn edit_stream(&self);
    fn delete_stream(&self);
    fn add_member(&self, member: Person);
    fn calc_silo(&self) -> u16;
}

impl Actions for Stream {
    fn new(
        name: String,
        description: String,
        stream_lead: Person,
        team: Vec<Person>,
        dependency: Vec<Dependency>,
        repository: String,
        readme: String,
        keywords: Vec<String>,
        documentation: String,
        homepage: String,
    ) -> Stream {
        Stream {
            name: name,
            description: description,
            stream_lead: stream_lead,
            team: team,
            dependency: dependency,
            repository: repository,
            homepage: homepage,
            documentation: documentation,
            readme: readme,
            keywords: keywords,
        }
    }
    
    fn status(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }

    fn edit_stream(&self) {
        // edit stream
    }

    fn delete_stream(&self) {
        // delete stream
    }

    fn add_member(&self, member: Person) {
        // add member to stream
    }

    fn calc_silo(&self) -> u16 {
        // ToDo: Generate an better silo_score for each stream
        // Naive implementation: sum up all dependencies
        self.dependency.len() as u16
    }
}

impl std::fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream: {}", self.name)
    }
}
