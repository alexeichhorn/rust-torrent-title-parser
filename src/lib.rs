use serde::{Deserialize, Serialize};
use thiserror::Error;

mod handlers;
mod parser;

pub use parser::Parser;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Failed to parse title")]
    ParseError(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedTitle {
    pub title: String,
    pub resolution: Option<String>,
    pub date: Option<String>,
    pub year: Option<i32>,
    pub ppv: bool,
    pub trash: bool,
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
    pub quality: Option<String>,
    pub bit_depth: Option<String>,
    pub hdr: Vec<String>,
    pub codec: Option<String>,
    pub audio: Vec<String>,
    pub channels: Vec<String>,
    pub group: Option<String>,
    pub container: Option<String>,
    pub volumes: Vec<i32>,
    pub seasons: Vec<i32>,
    pub episodes: Vec<i32>,
    pub episode_code: Option<String>,
    pub complete: bool,
    pub languages: Vec<String>,
    pub dubbed: bool,
    pub site: Option<String>,
    pub extension: Option<String>,
    pub subbed: bool,
    pub documentary: bool,
    pub upscaled: bool,
}

pub fn parse_title(raw_title: &str) -> Result<ParsedTitle, ParserError> {
    let mut parser = Parser::new();
    handlers::add_default_handlers(&mut parser);
    parser.parse(raw_title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let result = parse_title("The Simpsons S01E01 1080p BluRay x265 HEVC 10bit AAC 5.1 Tigole").unwrap();
        assert_eq!(result.title, "The Simpsons");
        assert_eq!(result.resolution, Some("1080p".to_string()));
        assert_eq!(result.quality, Some("bluray".to_string()));
        assert_eq!(result.codec, Some("x265".to_string()));
        assert!(result.audio.contains(&"aac".to_string()));
        assert!(result.channels.contains(&"5.1".to_string()));
        assert_eq!(result.seasons, vec![1]);
        assert_eq!(result.episodes, vec![1]);
        assert_eq!(result.group, Some("Tigole".to_string()));
    }
}
