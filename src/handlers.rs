use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    handler_wrapper::{Handler, RegexHandlerOptions},
    ParsedTitle,
};

lazy_static! {
    // Resolution patterns
    static ref RESOLUTION_PATTERN: Regex = Regex::new(r"(?i)\b(4k|2160p|1440p|1080p|720p|480p|360p|240p)\b").unwrap();

    // Quality patterns
    static ref QUALITY_PATTERN: Regex = Regex::new(r"(?i)\b(bluray|bdrip|brrip|webdl|webrip|hdtv|pdtv|dvdrip|dvdscr|web)\b").unwrap();

    // Codec patterns
    static ref CODEC_PATTERN: Regex = Regex::new(r"(?i)\b(x264|x265|h264|h265|hevc|xvid|divx|avc)\b").unwrap();

    // Audio patterns
    static ref AUDIO_PATTERN: Regex = Regex::new(r"(?i)\b(aac|ac3|dts|dd5\.1|dolby|truehd|atmos)\b").unwrap();

    // Channel patterns
    static ref CHANNEL_PATTERN: Regex = Regex::new(r"(?i)\b(2\.0|5\.1|7\.1)\b").unwrap();

    // Season/Episode patterns
    static ref SEASON_PATTERN: Regex = Regex::new(r"(?i)s(\d{1,2})").unwrap();
    static ref EPISODE_PATTERN: Regex = Regex::new(r"(?i)e(\d{1,3})").unwrap();

    // Language patterns
    static ref LANGUAGE_PATTERN: Regex = Regex::new(r"(?i)\b(english|spanish|french|german|italian|russian|japanese|korean|chinese|hindi)\b").unwrap();

    // Group patterns
    static ref GROUP_PATTERN: Regex = Regex::new(r"(?i)-(\w+)$").unwrap();
}

pub fn resolution_handler(title: &str) -> Option<(String, String)> {
    RESOLUTION_PATTERN
        .captures(title)
        .map(|caps| ("resolution".to_string(), caps[1].to_lowercase()))
}

pub fn quality_handler(title: &str) -> Option<(String, String)> {
    QUALITY_PATTERN
        .captures(title)
        .map(|caps| ("quality".to_string(), caps[1].to_lowercase()))
}

pub fn codec_handler(title: &str) -> Option<(String, String)> {
    CODEC_PATTERN
        .captures(title)
        .map(|caps| ("codec".to_string(), caps[1].to_lowercase()))
}

pub fn audio_handler(title: &str) -> Option<(String, String)> {
    AUDIO_PATTERN
        .captures(title)
        .map(|caps| ("audio".to_string(), caps[1].to_lowercase()))
}

pub fn channel_handler(title: &str) -> Option<(String, String)> {
    CHANNEL_PATTERN
        .captures(title)
        .map(|caps| ("channels".to_string(), caps[1].to_string()))
}

pub fn season_handler(title: &str) -> Option<(String, String)> {
    SEASON_PATTERN
        .captures(title)
        .map(|caps| ("seasons".to_string(), caps[1].to_string()))
}

pub fn episode_handler(title: &str) -> Option<(String, String)> {
    EPISODE_PATTERN
        .captures(title)
        .map(|caps| ("episodes".to_string(), caps[1].to_string()))
}

pub fn language_handler(title: &str) -> Option<(String, String)> {
    LANGUAGE_PATTERN
        .captures(title)
        .map(|caps| ("languages".to_string(), caps[1].to_lowercase()))
}

pub fn group_handler(title: &str) -> Option<(String, String)> {
    GROUP_PATTERN.captures(title).map(|caps| ("group".to_string(), caps[1].to_string()))
}

// commonly used transform functions
mod transforms {
    pub fn true_if_found(value: &str, _: &bool) -> Option<bool> {
        if value.is_empty() {
            None
        } else {
            Some(true)
        }
    }

    pub fn replace_value(value: &'static str) -> impl Fn(&str) -> String {
        move |input_value: &str| -> String {
            let mut result = value.to_string();
            result = result.replace("$1", input_value);
            result
        }
    }

    pub fn uniq_concat(value: &str, result: &Vec<String>) -> Option<Vec<String>> {
        let mut result = result.clone();
        if result.contains(&value.to_string()) {
            return Some(result);
        }
        result.push(value.to_string());
        Some(result)
    }

    pub fn chain_transforms<T, F1, F2, R1>(transform1: F1, transform2: F2) -> impl Fn(&str, &T) -> Option<T>
    where
        F1: Fn(&str) -> R1,
        F2: Fn(&str, &T) -> Option<T>,
        R1: AsRef<str>,
    {
        move |value: &str, state: &T| {
            let intermediate = transform1(value);
            transform2(intermediate.as_ref(), state)
        }
    }
}

pub fn add_default_handlers(parser: &mut super::Parser) {
    // Adult
    parser.add_handler(Handler::from_regex(
        "adult",
        |t| &mut t.adult,
        Regex::new(r"(?i)\b(?:xxx|xx)\b").unwrap(), // (?i) = case insensitive
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    // TODO: add adult keyword pattern here

    // Scene
    parser.add_handler(Handler::from_regex(
        "scene",
        |t| &mut t.scene,
        Regex::new(r"(\b\d{3,4}p\b.*[_. ]WEB[_. ][^D][^L]\b|\b-(?:CAKES|GGEZ|GGWP|GLHF|GOSSIP|NAISU|KOGI|PECULATE|SLOT|EDITH|ETHEL|ELEANOR|B2B|SPAMnEGGS|FTP|DiRT|SYNCOPY|BAE|SuccessfulCrab|NHTFS|SURCODE|B0MBARDIERS)\b)").unwrap(), // removed positive/negative lookahead (compated to Python version)
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));

    /*
       # Extras (This stuff can be trashed)
       parser.add_handler("extras", regex.compile(r"\bNCED\b", regex.IGNORECASE), uniq_concat(value("NCED")), {"remove": True})
       parser.add_handler("extras", regex.compile(r"\bNCOP\b", regex.IGNORECASE), uniq_concat(value("NCOP")), {"remove": True})
       parser.add_handler("extras", regex.compile(r"\b(?:Deleted[ .-]*)?Scene(?:s)?\b", regex.IGNORECASE), uniq_concat(value("Deleted Scene")), {"remove": False})
       parser.add_handler("extras", regex.compile(r"(?:(?<=\b(?:19\d{2}|20\d{2})\b.*)\b(?:Featurettes?)\b|\bFeaturettes?\b(?!.*\b(?:19\d{2}|20\d{2})\b))", regex.IGNORECASE), uniq_concat(value("Featurette")), {"skipFromTitle": True, "remove": False})
       parser.add_handler("extras", regex.compile(r"(?:(?<=\b(?:19\d{2}|20\d{2})\b.*)\b(?:Sample)\b|\b(?:Sample)\b(?!.*\b(?:19\d{2}|20\d{2})\b))", regex.IGNORECASE), uniq_concat(value("Sample")), {"skipFromTitle": True, "remove": False})
       parser.add_handler("extras", regex.compile(r"(?:(?<=\b(?:19\d{2}|20\d{2})\b.*)\b(?:Trailers?)\b|\bTrailers?\b(?!.*\b(?:19\d{2}|20\d{2}|.(Park|And))\b))", regex.IGNORECASE), uniq_concat(value("Trailer")), {"skipFromTitle": True, "remove": False})
    */

    // Extras (this stuff can be trashed)
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\bNCED\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("NCED"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
}
