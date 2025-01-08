use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::ParsedTitle;

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

pub struct Handler {
    name: String,
    handler: Box<dyn Fn(HandlerContext) -> Option<HandlerResult>>,
}

impl Handler {
    pub fn new_old(name: String, handler: Box<dyn Fn(HandlerContext) -> Option<HandlerResult>>) -> Self {
        Handler { name, handler }
    }

    pub fn new<F>(name: &str, handler: F) -> Self
    where
        F: Fn(HandlerContext) -> Option<HandlerResult> + 'static,
    {
        Handler::new_old(name.to_string(), Box::new(handler))
    }

    /*

    def handler(context: Dict[str, Any]) -> Union[Dict[str, Any], None]:
       title = context["title"]
       result = context["result"]
       matched = context["matched"]

       if name in result and options.get("skipIfAlreadyFound", False):
           return None
       if DEBUG_HANDLER is True or (type(DEBUG_HANDLER) is str and DEBUG_HANDLER in name):
           print(name, "Try to match " + title, "To " + reg_exp.pattern)
       match = reg_exp.search(title)
       if DEBUG_HANDLER is True or (type(DEBUG_HANDLER) is str and DEBUG_HANDLER in name):
           print("Matched " + str(match))
       if match:
           raw_match = match.group(0)
           clean_match = match.group(1) if len(match.groups()) >= 1 else raw_match
           sig = inspect.signature(transformer)
           param_count = len(sig.parameters)
           transformed = transformer(clean_match or raw_match, *([result.get(name)] if param_count > 1 else []))
           if type(transformed) is str:
               transformed = transformed.strip()

           before_title_match = BEFORE_TITLE_MATCH_REGEX.match(title)
           is_before_title = before_title_match is not None and raw_match in before_title_match.group(1)

           other_matches = {k: v for k, v in matched.items() if k != name}
           is_skip_if_first = options.get("skipIfFirst", False) and other_matches and all(match.start() < other_matches[k]["match_index"] for k in other_matches)

           if transformed is not None and not is_skip_if_first:
               matched[name] = matched.get(name, {"raw_match": raw_match, "match_index": match.start()})
               result[name] = options.get("value", transformed)
               return {"raw_match": raw_match, "match_index": match.start(), "remove": options.get("remove", False), "skip_from_title": is_before_title or options.get("skipFromTitle", False)}
       return None
    */
    pub fn from_regex<T: PropertyIsSet>(
        name: &'static str,
        accessor: impl Fn(&mut ParsedTitle) -> &mut T + 'static,
        regex: Regex,
        transform: impl Fn(&str, &T) -> Option<T> + 'static,
        options: RegexHandlerOptions,
    ) -> Self {
        let handler = Box::new(move |context: HandlerContext| {
            let field = accessor(context.result);
            if field.is_set() && options.skip_if_already_found {
                return None;
            }

            if let Some(captures) = regex.captures(context.title) {
                let m = captures.get(0).unwrap();
                let raw_match = m.as_str(); // will always succeed (as it is equal to whole match)
                let clean_match = captures.get(1).map(|m| m.as_str()).unwrap_or(raw_match);

                let Some(transformed) = transform(clean_match, field) else {
                    return None;
                };

                let before_title_match = BEFORE_TITLE_MATCH_REGEX.captures(context.title);
                let is_before_title = if let Some(before_title_match) = before_title_match {
                    before_title_match.get(1).unwrap().as_str().contains(raw_match)
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

// region:

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
