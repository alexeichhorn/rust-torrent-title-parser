use std::str::FromStr;

use chrono::NaiveDate;
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SANITIZER_REGEX: Regex = Regex::new(r"\W+").unwrap();
}

pub fn identity<'a>(value: &'a str, _: &Option<String>) -> Option<Option<String>> {
    Some(Some(value.to_string()))
}

pub fn uppercase(value: &str, _: &Option<String>) -> Option<Option<String>> {
    Some(Some(value.to_uppercase()))
}

pub fn lowercase(value: &str, _: &Option<String>) -> Option<Option<String>> {
    Some(Some(value.to_lowercase()))
}

pub fn true_if_found(value: &str, _: &bool) -> Option<bool> {
    if value.is_empty() {
        None
    } else {
        Some(true)
    }
}

pub fn parse<T: FromStr>(value: &str, _: &Option<T>) -> Option<Option<T>> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        match value.parse::<T>() {
            Ok(result) => Some(Some(result)),
            Err(_) => None,
        }
    }
}

/// note: `$1` is replaced with the input value
pub fn value(value: &'static str) -> impl Fn(&str, &Option<String>) -> Option<Option<String>> {
    move |input_value: &str, _| {
        let mut result = value.to_string();
        result = result.replace("$1", input_value);
        Some(Some(result))
    }
}

/// note: `$1` is replaced with the input value
pub fn replace_value(value: &'static str) -> impl Fn(&str) -> String {
    move |input_value: &str| -> String {
        let mut result = value.to_string();
        result = result.replace("$1", input_value);
        result
    }
}

fn convert_months(date_str: &str) -> String {
    let mut result = date_str.to_string();

    // Map of full month names to shortened forms
    lazy_static! {
        static ref MONTH_MAPPING: [(Regex, &'static str); 12] = [
            (Regex::new(r"(?i)\bJanu\b").unwrap(), "Jan"),
            (Regex::new(r"(?i)\bFebr\b").unwrap(), "Feb"),
            (Regex::new(r"(?i)\bMarc\b").unwrap(), "Mar"),
            (Regex::new(r"(?i)\bApri\b").unwrap(), "Apr"),
            (Regex::new(r"(?i)\bMay\b").unwrap(), "May"),
            (Regex::new(r"(?i)\bJune\b").unwrap(), "Jun"),
            (Regex::new(r"(?i)\bJuly\b").unwrap(), "Jul"),
            (Regex::new(r"(?i)\bAugu\b").unwrap(), "Aug"),
            (Regex::new(r"(?i)\bSept\b").unwrap(), "Sep"),
            (Regex::new(r"(?i)\bOcto\b").unwrap(), "Oct"),
            (Regex::new(r"(?i)\bNove\b").unwrap(), "Nov"),
            (Regex::new(r"(?i)\bDece\b").unwrap(), "Dec"),
        ];
    }

    // Replace each full month name with its shortened form
    for (month, shortened) in MONTH_MAPPING.iter() {
        result = month.replace_all(&result, *shortened).to_string();
    }

    result
}

/// Return a transformer that parses dates using the specified format
///
/// :param date_format: The date format to use for parsing (e.g. "YYYY MM DD")
/// :return: The transformer function.
pub fn date_from_format(format: &'static str) -> impl Fn(&str, &Option<String>) -> Option<Option<String>> {
    move |input_value: &str, _| {
        let sanitized = SANITIZER_REGEX.replace_all(input_value, " ").trim().to_string();
        let sanitized = convert_months(&sanitized);

        println!("input_value: '{}' sanitized: '{}' for format: '{}'", input_value, sanitized, format);

        let date = NaiveDate::parse_from_str(&sanitized, format).ok()?;

        Some(Some(date.format("%Y-%m-%d").to_string())) // normalize to YYYY-MM-DD
    }
}

/// Return a transformer that parses dates using the specified formats
pub fn date_from_formats(formats: &'static [&'static str]) -> impl Fn(&str, &Option<String>) -> Option<Option<String>> {
    let format_functions: Vec<_> = formats.iter().map(|format| date_from_format(format)).collect();

    move |input_value: &str, existing| {
        for format_function in &format_functions {
            let result = format_function(input_value, existing);
            if result.is_some() {
                return result;
            }
        }
        None
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

/// Transform the resolution string to a standardized format (e.g. 1080p)
pub fn resolution_transform(value: &str, _: &Option<String>) -> Option<Option<String>> {
    let input_value = value.to_lowercase();

    if input_value.contains("2160") || input_value.contains("4k") {
        return Some(Some("2160p".to_string()));
    } else if input_value.contains("1440") || input_value.contains("2k") {
        return Some(Some("1440p".to_string()));
    } else if input_value.contains("1080") {
        return Some(Some("1080p".to_string()));
    } else if input_value.contains("720") {
        return Some(Some("720p".to_string()));
    } else if input_value.contains("480") {
        return Some(Some("480p".to_string()));
    } else if input_value.contains("360") {
        return Some(Some("360p".to_string()));
    } else if input_value.contains("240") {
        return Some(Some("240p".to_string()));
    }
    None
}

// region: Chaining

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

// endregion
