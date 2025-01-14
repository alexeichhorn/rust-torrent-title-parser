use std::collections::HashMap;

use lazy_static::lazy_static;
use regress::Regex;

use crate::{extensions::regex::RegexStringExt, Codec, ParsedTitle};

#[derive(Debug)]
pub struct Match {
    pub raw_match: String,
    pub match_index: usize,
}

pub struct HandlerContext<'a> {
    pub title: &'a str,
    pub result: &'a mut ParsedTitle,
    pub matched: &'a mut HashMap<String, Match>,
    // end_of_title: &'a mut usize,
}

#[derive(Debug)]
pub struct HandlerResult {
    pub raw_match: String,
    pub match_index: usize,
    pub remove: bool,
    pub skip_from_title: bool,
}

pub struct RegexHandlerOptions {
    pub skip_if_already_found: bool,
    pub skip_from_title: bool,
    pub skip_if_first: bool,
    pub remove: bool,
}

impl Default for RegexHandlerOptions {
    fn default() -> Self {
        Self {
            skip_if_already_found: true,
            skip_from_title: false,
            skip_if_first: false,
            remove: false,
        }
    }
}

lazy_static! {
    static ref BEFORE_TITLE_MATCH_REGEX: Regex = Regex::new(r"^\[(.*?)\]").unwrap();
}

type HandlerFn = dyn Fn(HandlerContext) -> Option<HandlerResult> + Send + Sync;

pub struct Handler {
    name: String,
    handler: Box<HandlerFn>,
}

impl Handler {
    pub fn new_old(name: String, handler: Box<HandlerFn>) -> Self {
        Handler { name, handler }
    }

    pub fn new<F>(name: &str, handler: F) -> Self
    where
        F: Fn(HandlerContext) -> Option<HandlerResult> + Send + Sync + 'static,
    {
        Handler::new_old(name.to_string(), Box::new(handler))
    }

    pub fn from_regex<T: PropertyIsSet + TrimIfString>(
        name: &'static str,
        accessor: impl Fn(&mut ParsedTitle) -> &mut T + Send + Sync + 'static,
        regex: Regex,
        transform: impl Fn(&str, &T) -> Option<T> + Send + Sync + 'static,
        options: RegexHandlerOptions,
    ) -> Self {
        let handler = Box::new(move |context: HandlerContext| {
            let field = accessor(context.result);
            if field.is_set() && options.skip_if_already_found {
                return None;
            }

            if let Some(m) = regex.find_str(context.title) {
                let raw_match = m.as_str(); // will always succeed (as it is equal to whole match)
                let clean_match = m.group(1).map(|m| m.as_str()).unwrap_or(raw_match);

                let Some(transformed) = transform(clean_match, field) else {
                    return None;
                };

                // If transformed is a string, strip whitespace
                let transformed = transformed.trim_if_string();

                let before_title_match = BEFORE_TITLE_MATCH_REGEX.find_str(context.title);
                let is_before_title = if let Some(before_title_match) = before_title_match {
                    before_title_match.group(1).unwrap().as_str().contains(raw_match)
                } else {
                    false
                };

                let other_matches = context
                    .matched
                    .iter()
                    .filter(|(k, _)| k.as_str() != name)
                    .collect::<HashMap<_, _>>();
                let is_skip_if_first =
                    options.skip_if_first && !other_matches.is_empty() && other_matches.iter().all(|(_, v)| m.start() < v.match_index);

                if !is_skip_if_first {
                    context.matched.insert(
                        name.to_string(),
                        Match {
                            raw_match: raw_match.to_string(),
                            match_index: m.start(),
                        },
                    );
                    // set the extracted data
                    *field = transformed;

                    return Some(HandlerResult {
                        raw_match: raw_match.to_string(),
                        match_index: m.start(),
                        remove: options.remove,
                        skip_from_title: is_before_title || options.skip_from_title,
                    });
                } else {
                    None
                }
            } else {
                None
            }
        });

        Self::new(&name, handler)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn call(&self, context: HandlerContext) -> Option<HandlerResult> {
        (self.handler)(context)
    }
}

/*
pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(&str) -> Option<(String, String)> + 'static,
    {
        self.handlers.push(Box::new(handler));
    } */

// region: PropertyIsSet

pub trait PropertyIsSet {
    fn is_set(&self) -> bool;
}

impl<T> PropertyIsSet for Option<T> {
    fn is_set(&self) -> bool {
        self.is_some()
    }
}

impl PropertyIsSet for bool {
    fn is_set(&self) -> bool {
        *self
    }
}

impl<T> PropertyIsSet for Vec<T> {
    fn is_set(&self) -> bool {
        !self.is_empty()
    }
}

impl PropertyIsSet for String {
    fn is_set(&self) -> bool {
        !self.is_empty()
    }
}

// endregion
// region: TrimIfString

pub trait TrimIfString {
    fn trim_if_string(self) -> Self;
}

impl TrimIfString for String {
    fn trim_if_string(self) -> String {
        self.trim().to_string()
    }
}

impl<'a> TrimIfString for &'a str {
    fn trim_if_string(self) -> &'a str {
        self.trim()
    }
}

impl TrimIfString for bool {
    fn trim_if_string(self) -> bool {
        self
    }
}

impl TrimIfString for i32 {
    fn trim_if_string(self) -> i32 {
        self
    }
}

impl TrimIfString for Codec {
    fn trim_if_string(self) -> Codec {
        self
    }
}

impl<T> TrimIfString for Vec<T> {
    fn trim_if_string(self) -> Vec<T> {
        self
    }
}

impl<T: TrimIfString> TrimIfString for Option<T> {
    fn trim_if_string(self) -> Option<T> {
        self.map(|s| s.trim_if_string())
    }
}

// endregion
