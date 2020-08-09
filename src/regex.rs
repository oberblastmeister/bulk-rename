use rayon::prelude::*;
use regex::Regex;

/// takes an iterator and filters all items that match the pattern
/// only uses contains for now, doesn't use regex matching
pub fn filter_matches(items: Vec<String>, pattern: Regex) -> Vec<String> {
    items
        .into_par_iter()
        .filter(|x| pattern.is_match(x))
        .collect()
}
