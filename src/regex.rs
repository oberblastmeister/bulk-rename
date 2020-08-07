/// takes an iterator and filters all items that match the pattern
/// only uses contains for now, doesn't use regex matching
pub fn find_pattern(
    items: impl ParallelIterator<Item = String>,
    pattern: Option<&String>,
) -> Vec<String> {
    items
        .into_par_iter()
        .filter(|x| {
            if let Some(pattern) = pattern {
                x.contains(pattern)
            } else {
                true
            }
        })
        .collect()
}
