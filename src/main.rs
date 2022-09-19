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
    stream: String,
    program: String,
    forum: String,
}

fn main() {
    // get the command line arguments
    let args = Cli::parse();

    //create a new stream
    let stream = streams::Stream::new(
        String::from(&args.stream),
    );

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

    // create a new forum and write it to a file
    match std::fs::create_dir_all("forums") {
        Ok(it) => it,
        Err(err) => panic!("Error creating directory: {}", err),
    };

    // Output the JSON to stdout
    let json = serde_json::to_string_pretty(&forum).unwrap();
    println!("{}", json);
    std::fs::write("./forums/output.json", serde_json::to_string_pretty(&forum).unwrap()).unwrap();
}
