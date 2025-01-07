use std::collections::HashMap;

use regex::Regex;

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

pub struct Handler {
    name: String,
    handler: Box<dyn Fn(&mut HandlerContext) -> Option<HandlerResult>>,
}

impl Handler {
    pub fn new_old(name: String, handler: Box<dyn Fn(&mut HandlerContext) -> Option<HandlerResult>>) -> Self {
        Handler { name, handler }
    }

    pub fn new<F>(name: &str, handler: F) -> Self
    where
        F: Fn(&mut HandlerContext) -> Option<HandlerResult> + 'static,
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
        name: &str,
        accessor: impl Fn(&mut ParsedTitle) -> &mut T + 'static,
        regex: Regex,
        transform: Option<Box<dyn Fn(&str) -> Option<String>>>,
        options: RegexHandlerOptions,
    ) -> Self {
        let handler = Box::new(move |context: &mut HandlerContext| {
            let field = accessor(context.result);
            if field.is_set() && options.skip_if_already_found {
                return None;
            }

            if let Some(captures) = regex.captures(context.title) {
                let raw_match = captures.get(0).unwrap().as_str();
                let clean_match = captures.get(1).map(|m| m.as_str()).unwrap_or(raw_match);

                todo!("Regex handlers not fully implemented yet");
            } else {
                None
            }
        });

        Self::new(name, handler)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn call(&self, context: &mut HandlerContext) -> Option<HandlerResult> {
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

trait PropertyIsSet {
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
