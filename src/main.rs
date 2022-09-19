#![allow(unused)]

use std::vec;

use crate::forums::Actions as forumactions;
use crate::programs::Actions as programactions;
use crate::streams::Actions as streamactions;
use clap::Parser;
use serde_json::*;

mod forums;
mod programs;
mod streams;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    forum: String,
    program: String,
    stream: String,
}

fn main() {
    // get the command line arguments
    let args = Cli::parse();

    //create a new stream
    let mut stream = streams::Stream::new(
        String::from(&args.stream),
        String::from("A project is any undertaking, carried out individually or collaboratively and possibly involving research or design, that is carefully planned to achieve a particular aim."),
        streams::Person::new(
            "Martin Maurer".to_string(),
            vec!["rust".to_string(), "python".to_string()],
        ),
        vec![streams::Person::new(
            "Nazar Hussain".to_string(),
            vec!["rust".to_string(), "python".to_string()],
        ), streams::Person::new(
            "Willem Olding".to_string(),
            vec!["rust".to_string(), "python".to_string()],
        )],
        "https://github.com/empea-careercriminal/concierge".to_string(),
        "The Readme is cool.".to_string(),
        vec!["web3".to_string(), "WebAssembly".to_string()],
        "https://www.google.de/documentition".to_string(),
        "https://www.notion.so/somepage".to_string(),
    );

    //debug print the stream
    stream.status();
    
    //create a new program
    let mut prog = programs::Program::new(
        String::from(&args.program),
        Vec::new()
    );
    prog.members.push(stream);

    //create a new forum
    let mut forum = forums::Forum::new(
        String::from(&args.forum),
        Vec::new(),
    );
    forum.members.push(prog);
    
    // create folder structure
    match std::fs::create_dir_all("forums") {
        Ok(it) => it,
        Err(err) => panic!("Error creating directory: {}", err),
    };
    
    // Generate Json
    let output_path = format!("forums/{}.json", forum.name);
    std::fs::write(output_path, serde_json::to_string_pretty(&forum).unwrap()).unwrap();
}
