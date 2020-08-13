use anyhow::{Result, Context};
use rayon::prelude::*;
use regex::Regex;

/// Filters all items in a vector that match the regex pattern.
pub fn filter_matches(items: Vec<String>, pattern: Option<&str>) -> Result<Vec<String>> {
    let regex;
    if let Some(pattern) = pattern {
        regex = Regex::new(pattern).context(format!("Failed to create regex with pattern `{}`", pattern))?
    } else {
        return Ok(items);
    }
        
    Ok(items
        .into_par_iter()
        .filter(|x| regex.is_match(x))
        .collect())
}
