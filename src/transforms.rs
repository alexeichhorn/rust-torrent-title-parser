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
