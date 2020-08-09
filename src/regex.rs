use rayon::prelude::*;
use regex::Regex;

/// Filters all items in a vector that match the regex pattern.
pub fn filter_matches(items: Vec<String>, pattern: Regex) -> Vec<String> {
    items
        .into_par_iter()
        .filter(|x| pattern.is_match(x))
        .collect()
}
