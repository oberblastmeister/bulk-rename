use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;

/// takes an iterator and filters all items that match the pattern
/// only uses contains for now, doesn't use regex matching
pub fn filter_matches(items: impl ParallelIterator<Item = String>, pattern: Regex) -> Vec<String> {
    items
        .into_par_iter()
        .filter(|x| pattern.is_match(x))
        .collect()
}

pub fn create_lazy_regex(pattern: &str) -> Result<Regex> {
    let mut regex = String::new();
    if !pattern.starts_with(".*") {
        regex.push_str(".*");
    }
    regex.push_str(r#pattern);
    if !pattern.ends_with(".*") {
        regex.push_str(".*");
    }
    Ok(Regex::new(&regex)?)
}
