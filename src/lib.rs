#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use bitstream_reader::Result as ReadResult;

pub use crate::demo::{
    message::MessageType,
    parser::{DemoParser, MatchState, MessageTypeAnalyser, Parse, ParseError, ParserState, Result},
    Demo, Stream,
};

pub mod demo;
