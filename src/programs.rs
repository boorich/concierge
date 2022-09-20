use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::format;

use crate::streams::Actions as streamactions;
use crate::streams::Stream;

#[derive(Debug, Serialize, Clone)]
pub struct Program {
    pub name: String,
    pub members: Vec<Stream>,
}

pub trait Actions {
    fn new(name: String, members: Vec<Stream>) -> Program;
    fn status(&self) -> String;
    fn calc_silo_score_per_stream(&self) -> Vec<u16>;
    // fn merge_streams(&self, stream1: Stream, stream2: Stream) -> Stream;
    // fn diff_streams(&self, stream: Stream, ratio: Stream) -> Stream;
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

    fn calc_silo_score_per_stream(&self) -> Vec<u16> {
        // ToDo: Generate an silo_score for each stream in the program
        // This could be done by evaluating the number of external dependencies

        let mut silo_score = vec![];
        for stream in &self.members {
            silo_score.push(streamactions::calc_silo(stream));
        }
        silo_score
    }

    // fn merge_streams(&self, stream1: Stream, stream2: Stream) -> Stream {
    //     let mut stream = Stream::new(
    //         stream1.name,
    //         stream1.description,
    //         stream1.stream_lead,
    //         stream1.team,
    //         stream1.repository,
    //         stream1.readme,
    //         stream1.keywords,
    //         stream1.documentation,
    //         stream1.homepage,
    //     );
    //     stream
    // }

    // fn diff_streams(&self, stream: Stream, ratio: Stream) -> Stream {
    //     let mut stream = Stream::new(
    //         stream.name,
    //         stream.description,
    //         stream.stream_lead,
    //         stream.team,
    //         stream.repository,
    //         stream.readme,
    //         stream.keywords,
    //         stream.documentation,
    //         stream.homepage,
    //     );
    //     stream
    // }
}
