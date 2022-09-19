use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, de};
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
#[derive(Debug)]
struct Dependency {
    deadline: chrono::DateTime<chrono::Utc>,
    budget: u32,
    expert: Person,
    code: Vec<String>,
    legal: Vec<String>, //optional field for legal documents
}
#[derive(Debug)]
struct Feature {
    name: String,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Stream {
    pub name: String,      // name of the stream
    pub description: String,   // description of the stream
    pub stream_lead: Person,   // stream lead
    pub team: Vec<Person>,     // team members
    pub repository: String,    // code repository
    pub readme: String,        // what is this stream about
    pub keywords: Vec<String>, // keywords for tagging
    pub documentation: String, // optional documentation
    pub homepage: String,      // optional homepage
}

pub trait Actions {
    fn new(
        name: String,
        description: String,
        stream_lead: Person,
        team: Vec<Person>,
        repository: String,
        readme: String,
        keywords: Vec<String>,
        documentation: String,
        homepage: String,
    ) -> Stream;
    fn status(&self);
    fn edit_stream(&self);
    fn delete_stream(&self);
}

impl Actions for Stream {
    fn new(
        name: String,
        description: String,
        stream_lead: Person,
        team: Vec<Person>,
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
}
