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
    pub fn bool_if_non_empty(value: &str, _: &bool) -> Option<bool> {
        if value.is_empty() {
            None
        } else {
            Some(true)
        }
    }
}

pub fn add_default_handlers(parser: &mut super::Parser) {
    // Adult
    parser.add_handler(Handler::from_regex(
        "adult",
        |t| &mut t.adult,
        Regex::new(r"(?i)\b(?:xxx|xx)\b").unwrap(), // (?i) = case insensitive
        transforms::bool_if_non_empty,
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));

    // parser.add_handler(resolution_handler);
    // parser.add_handler(quality_handler);
    // parser.add_handler(codec_handler);
    // parser.add_handler(audio_handler);
    // parser.add_handler(channel_handler);
    // parser.add_handler(season_handler);
    // parser.add_handler(episode_handler);
    // parser.add_handler(language_handler);
    // parser.add_handler(group_handler);
}
