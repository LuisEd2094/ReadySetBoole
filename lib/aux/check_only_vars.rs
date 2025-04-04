use regex::Regex;

pub fn check_only_vars(input: &str) -> bool {
    Regex::new(r"^[A-Z!&|ˆ>=]+$").unwrap().is_match(input)
}
