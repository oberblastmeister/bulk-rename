use rayon::prelude::*;
use anyhow::Result;
use regex::Regex;

/// takes an iterator and filters all items that match the pattern
/// only uses contains for now, doesn't use regex matching
pub fn find_match(items: impl ParallelIterator<Item = String>, pattern: Regex) -> Vec<String> {
    items
        .into_par_iter()
        .filter(|x| pattern.is_match(x))
        .collect()
}

pub fn create_regex(pattern: &str) -> Result<Regex> {
    let mut regex = String::new();
    if !pattern.starts_with(".*") {
        regex.push_str(".*");
    }
    regex.push_str(pattern);
    if !pattern.ends_with(".*") {
        regex.push_str(".*");
    }
    Ok(Regex::new(&regex)?)
}
