use pcre2::bytes::{CaptureMatches, Captures, Match, Matches, Regex, RegexBuilder};
use pcre2::Error;

pub trait RegexStringExt
where
    Self: Sized,
{
    fn new_utf(pattern: &str) -> Result<Self, Error>;
    fn is_match_str(&self, subject: &str) -> Result<bool, Error>;
    fn find_str<'s>(&self, subject: &'s str) -> Result<Option<Match<'s>>, Error>;
    fn find_iter_str<'r, 's>(&'r self, subject: &'s str) -> Matches<'r, 's>;
    fn captures_str<'s>(&self, subject: &'s str) -> Result<Option<Captures<'s>>, Error>;
    fn captures_iter_str<'r, 's>(&'r self, subject: &'s str) -> CaptureMatches<'r, 's>;
}

impl RegexStringExt for Regex {
    fn new_utf(pattern: &str) -> Result<Self, Error> {
        RegexBuilder::new().utf(true).build(pattern)
    }

    fn is_match_str(&self, subject: &str) -> Result<bool, Error> {
        self.is_match(subject.as_bytes())
    }

    fn find_str<'s>(&self, subject: &'s str) -> Result<Option<Match<'s>>, Error> {
        self.find(subject.as_bytes())
    }

    fn find_iter_str<'r, 's>(&'r self, subject: &'s str) -> Matches<'r, 's> {
        self.find_iter(subject.as_bytes())
    }

    fn captures_str<'s>(&self, subject: &'s str) -> Result<Option<Captures<'s>>, Error> {
        self.captures(subject.as_bytes())
    }

    fn captures_iter_str<'r, 's>(&'r self, subject: &'s str) -> CaptureMatches<'r, 's> {
        self.captures_iter(subject.as_bytes())
    }
}

// - Match

pub trait MatchExt<'a> {
    fn as_str(&self) -> &'a str;
}

impl<'a> MatchExt<'a> for Match<'a> {
    fn as_str(&self) -> &'a str {
        std::str::from_utf8(self.as_bytes()).unwrap_or_default()
    }
}
