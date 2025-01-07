use std::collections::HashMap;

use crate::ParsedTitle;

#[derive(Debug)]
pub struct Match {
    pub raw_match: String,
    pub match_index: usize,
    pub remove: bool,
}

pub struct HandlerContext<'a> {
    pub title: &'a str,
    pub result: &'a mut ParsedTitle,
    pub matched: &'a mut HashMap<String, Match>,
    // end_of_title: &'a mut usize,
}

pub struct HandlerResult {
    pub raw_match: String,
    pub match_index: usize,
    pub remove: bool,
    pub skip_from_title: bool,
}

pub struct Handler {
    name: String,
    handler: Box<dyn Fn(&HandlerContext) -> Option<HandlerResult>>,
}

impl Handler {
    pub fn new(name: String, handler: Box<dyn Fn(&HandlerContext) -> Option<HandlerResult>>) -> Self {
        Handler { name, handler }
    }

    pub fn from_regex(name: String, regex: &str) -> Self {
        todo!()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn call(&self, context: &HandlerContext) -> Option<HandlerResult> {
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
