use crate::ParsedTitle;
use crate::ParserError;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CLEAN_TITLE_REGEX: Regex = Regex::new(r"_+").unwrap();
    static ref MOVIE_REGEX: Regex = Regex::new(r"(?i)[\[\(]movie[\]\)]").unwrap();
    static ref RUSSIAN_CAST_REGEX: Regex = Regex::new(r"\([^)]*[\u0400-\u04ff][^)]*\)$|/.*\([^)]*\)$").unwrap();
    static ref EMPTY_BRACKETS_REGEX: Regex = Regex::new(r"\(\s*\)|\[\s*\]|\{\s*\}").unwrap();
    static ref MP3_REGEX: Regex = Regex::new(r"\bmp3$").unwrap();
    static ref SPACING_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

#[derive(Debug)]
struct Match {
    raw_match: String,
    match_index: usize,
    remove: bool,
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

    fn clean_title(&self, title: &str) -> String {
        let mut cleaned = title.to_string();
        cleaned = CLEAN_TITLE_REGEX.replace_all(&cleaned, " ").to_string();
        cleaned = MOVIE_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = RUSSIAN_CAST_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = EMPTY_BRACKETS_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = MP3_REGEX.replace_all(&cleaned, "").to_string();
        cleaned.trim().to_string()
    }

    pub fn parse(&self, raw_title: &str) -> Result<ParsedTitle, ParserError> {
        let mut result = ParsedTitle::default();
        let mut title = raw_title.to_string();
        let mut matched: HashMap<String, Match> = HashMap::new();
        let mut end_of_title = title.len();

        // Basic title cleaning
        title = CLEAN_TITLE_REGEX.replace_all(&title, " ").to_string();

        // Apply handlers and track matches
        for handler in &self.handlers {
            if let Some((key, value)) = handler(&title) {
                // Find the match in the original title
                if let Some(match_index) = title.find(&value) {
                    matched.insert(
                        key.clone(),
                        Match {
                            raw_match: value.clone(),
                            match_index,
                            remove: true,
                        },
                    );

                    // Update end_of_title if this match is earlier
                    if match_index > 0 && match_index < end_of_title {
                        end_of_title = match_index;
                    }

                    // Process the value
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
        }

        // Clean the title by taking only the part before the first match
        let title = title[..end_of_title].to_string();
        result.title = self.clean_title(&title);

        Ok(result)
    }
}
