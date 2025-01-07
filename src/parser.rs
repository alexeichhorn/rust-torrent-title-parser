use crate::ParsedTitle;
use crate::ParserError;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CLEAN_TITLE_REGEX: Regex = Regex::new(r"_").unwrap();
    static ref MOVIE_REGEX: Regex = Regex::new(r"(?i)[\[\(]movie[\]\)]").unwrap();
    static ref RUSSIAN_CAST_REGEX: Regex = Regex::new(r"\([^)]*[\u0400-\u04ff][^)]*\)$|/.*\([^)]*\)$").unwrap();
    static ref EMPTY_BRACKETS_REGEX: Regex = Regex::new(r"\(\s*\)|\[\s*\]|\{\s*\}").unwrap();
    static ref MP3_REGEX: Regex = Regex::new(r"\bmp3$").unwrap();
    static ref SPACING_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

pub struct Parser {
    handlers: Vec<Box<dyn Fn(&str) -> Option<(String, String)>>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { handlers: Vec::new() }
    }

    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(&str) -> Option<(String, String)> + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn parse(&self, raw_title: &str) -> Result<ParsedTitle, ParserError> {
        let mut result = ParsedTitle::default();
        let mut title = raw_title.to_string();

        // Clean title
        title = CLEAN_TITLE_REGEX.replace_all(&title, " ").to_string();
        title = MOVIE_REGEX.replace_all(&title, "").to_string();
        title = RUSSIAN_CAST_REGEX.replace_all(&title, "").to_string();
        title = EMPTY_BRACKETS_REGEX.replace_all(&title, "").to_string();
        title = MP3_REGEX.replace_all(&title, "").to_string();
        title = SPACING_REGEX.replace_all(&title, " ").trim().to_string();

        // Apply handlers
        for handler in &self.handlers {
            if let Some((key, value)) = handler(&title) {
                match key.as_str() {
                    "resolution" => result.resolution = Some(value),
                    "quality" => result.quality = Some(value),
                    "codec" => result.codec = Some(value),
                    "audio" => result.audio.push(value),
                    "channels" => result.channels.push(value),
                    "languages" => result.languages.push(value),
                    "seasons" => {
                        if let Ok(season) = value.parse::<i32>() {
                            result.seasons.push(season);
                        }
                    }
                    "episodes" => {
                        if let Ok(episode) = value.parse::<i32>() {
                            result.episodes.push(episode);
                        }
                    }
                    "group" => result.group = Some(value),
                    _ => {}
                }
            }
        }

        result.title = title;
        Ok(result)
    }
}
