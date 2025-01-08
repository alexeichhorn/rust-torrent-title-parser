pub fn identity<'a>(value: &'a str, _: &Option<String>) -> Option<Option<String>> {
    Some(Some(value.to_string()))
}

pub fn uppercase(value: &str, _: &Option<String>) -> Option<Option<String>> {
    Some(Some(value.to_uppercase()))
}

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
