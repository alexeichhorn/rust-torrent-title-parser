use crate::handler_wrapper::Handler;
use crate::handler_wrapper::HandlerContext;
use crate::handler_wrapper::Match;
use crate::ParsedTitle;
use crate::ParserError;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const CURLY_BRACKETS: (&str, &str) = ("{", "}");
const SQUARE_BRACKETS: (&str, &str) = ("[", "]");
const PARENTHESES: (&str, &str) = ("(", ")");
const BRACKETS: [(&str, &str); 3] = [CURLY_BRACKETS, SQUARE_BRACKETS, PARENTHESES];

lazy_static! {
    static ref CLEAN_TITLE_REGEX: Regex = Regex::new(r"_+").unwrap();
    static ref MOVIE_REGEX: Regex = Regex::new(r"[\[\(]movie[\)\]]").unwrap();
    static ref RUSSIAN_CAST_REGEX: Regex = Regex::new(r"\([^)]*[\u0400-\u04ff][^)]*\)$|/[^/]*\([^)]*\)$").unwrap();
    static ref ALT_TITLES_REGEX: Regex = Regex::new(r"[^/|(]*[{NON_ENGLISH_CHARS}][^/|]*[/|]|[/|][^/|(]*[{NON_ENGLISH_CHARS}][^/|]*").unwrap();
    static ref NOT_ONLY_NON_ENGLISH_REGEX: Regex = Regex::new(r"[a-zA-Z][^{NON_ENGLISH_CHARS}]+[{NON_ENGLISH_CHARS}].*[{NON_ENGLISH_CHARS}]|[{NON_ENGLISH_CHARS}].*[{NON_ENGLISH_CHARS}][^{NON_ENGLISH_CHARS}]+[a-zA-Z]").unwrap();
    static ref NOT_ALLOWED_SYMBOLS_AT_START_AND_END: Regex = Regex::new(r"^[^\w{NON_ENGLISH_CHARS}#\[【★]+|[ \-:/\\\[\|{(#$&^]+$").unwrap();
    static ref REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END: Regex = Regex::new(r"^[^\w{NON_ENGLISH_CHARS}#]+|]$").unwrap();
    static ref REDUNDANT_SYMBOLS_AT_END: Regex = Regex::new(r"[ \-:./\\]+$").unwrap();
    static ref EMPTY_BRACKETS_REGEX: Regex = Regex::new(r"\(\s*\)|\[\s*\]|\{\s*\}").unwrap();
    static ref PARANTHESES_WITHOUT_CONTENT: Regex = Regex::new(r"\(\W*\)|\[\W*\]|\{\W*\}").unwrap();
    static ref STAR_REGEX_1: Regex = Regex::new(r"^[\[【★].*[\]】★][ \.]?(.+)").unwrap();
    static ref STAR_REGEX_2: Regex = Regex::new(r"(.+)[ \.]?[\[【★].*[\]】★]$").unwrap();
    static ref MP3_REGEX: Regex = Regex::new(r"\bmp3$").unwrap();
    static ref SPACING_REGEX: Regex = Regex::new(r"\s+").unwrap();
    static ref DOT_REGEX: Regex = Regex::new(r"\.").unwrap();
}

pub struct Parser {
    handlers: Vec<Handler>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { handlers: Vec::new() }
    }

    pub fn add_handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }

    fn clean_title(&self, title: &str) -> String {
        let mut cleaned = title.to_string();
        cleaned = cleaned.replace("_", " ");
        cleaned = MOVIE_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = NOT_ALLOWED_SYMBOLS_AT_START_AND_END.replace_all(&cleaned, "").to_string();
        cleaned = RUSSIAN_CAST_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = STAR_REGEX_1.replace_all(&cleaned, r"\1").to_string();
        cleaned = STAR_REGEX_2.replace_all(&cleaned, r"\1").to_string();
        cleaned = ALT_TITLES_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = NOT_ONLY_NON_ENGLISH_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END.replace_all(&cleaned, "").to_string();
        cleaned = EMPTY_BRACKETS_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = MP3_REGEX.replace_all(&cleaned, "").to_string();
        cleaned = PARANTHESES_WITHOUT_CONTENT.replace_all(&cleaned, "").to_string();

        // Remove brackets if only one is present
        for (open_bracket, close_bracket) in BRACKETS {
            if cleaned.matches(open_bracket).count() != cleaned.matches(close_bracket).count() {
                cleaned = cleaned.replace(open_bracket, "").replace(close_bracket, "");
            }
        }

        if !cleaned.contains(" ") && cleaned.contains(".") {
            cleaned = DOT_REGEX.replace_all(&cleaned, " ").to_string();
        }

        cleaned = REDUNDANT_SYMBOLS_AT_END.replace_all(&cleaned, "").to_string();
        cleaned = SPACING_REGEX.replace_all(&cleaned, " ").to_string();
        cleaned = cleaned.trim().to_string();
        cleaned
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
            let match_result = handler.call(&HandlerContext {
                title: &title,
                result: &mut result,
                matched: &mut matched,
                // end_of_title: &mut end_of_title,
            });

            // println!("match result for {}: {:?}", handler.get_name(), match_result);

            let Some(match_result) = match_result else {
                continue;
            };

            if match_result.remove {
                title = format!(
                    "{}{}",
                    &title[..match_result.match_index],
                    &title[match_result.match_index + match_result.raw_match.len()..]
                );
            }
            if !match_result.skip_from_title && 1 < match_result.match_index && match_result.match_index < end_of_title {
                end_of_title = match_result.match_index;
            }
            if match_result.remove && match_result.skip_from_title && match_result.match_index < end_of_title {
                end_of_title -= match_result.raw_match.len();
            }
        }

        // Clean the title by taking only the part before the first match
        let title = title[..end_of_title].to_string();
        result.title = self.clean_title(&title);

        Ok(result)
    }
}
