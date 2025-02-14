use thiserror::Error;

mod extensions;
mod handler_wrapper;
mod handlers;
mod parser;
mod transforms;
mod types;

pub use parser::Parser;
pub use types::{Codec, Language, Network, Quality};

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Failed to parse title")]
    ParseError(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedTitle {
    pub title: String,
    pub resolution: Option<String>,
    pub date: Option<String>,
    pub year: Option<i32>,
    pub ppv: bool,
    pub trash: bool,
    pub adult: bool,
    pub edition: Option<String>,
    pub extended: bool,
    pub convert: bool,
    pub hardcoded: bool,
    pub proper: bool,
    pub repack: bool,
    pub retail: bool,
    pub remastered: bool,
    pub unrated: bool,
    pub region: Option<String>,
    pub quality: Option<Quality>,
    pub bitrate: Option<String>,
    pub bit_depth: Option<String>,
    pub hdr: Vec<String>,
    pub codec: Option<Codec>,
    pub audio: Vec<String>,
    pub channels: Vec<String>,
    pub group: Option<String>,
    pub container: Option<String>,
    pub volumes: Vec<i32>,
    pub seasons: Vec<i32>,
    pub episodes: Vec<i32>,
    pub episode_code: Option<String>,
    pub complete: bool,
    pub languages: Vec<Language>,
    pub dubbed: bool,
    pub site: Option<String>,
    pub extension: Option<String>,
    pub subbed: bool,
    pub documentary: bool,
    pub upscaled: bool,
    pub is_3d: bool,
    pub extras: Vec<String>,
    pub size: Option<String>,
    pub network: Option<Network>,
    pub scene: bool,
}

pub fn parse_title(raw_title: &str) -> Result<ParsedTitle, ParserError> {
    let parser = Parser::default();
    parser.parse(raw_title)
}
