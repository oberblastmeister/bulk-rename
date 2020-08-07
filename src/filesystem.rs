use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;

pub fn get_sorted_paths(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(dir)?.collect::<Vec<_>>()
        .into_par_iter()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_unstable();

    Ok(entries)
}

pub fn convert_paths(paths: Vec<PathBuf>) -> impl ParallelIterator<Item = Result<String>> {
    paths.into_par_iter().map(|p| {
        p.into_os_string()
            .into_string()
            .map_err(|_| anyhow!("Could not convert OsString to a utf-8 String"))
    })
}

pub fn find_pattern(items: impl ParallelIterator<Item = String>, pattern: &str) -> String {
    let items_match_pattern: Vec<String> =
        items.into_par_iter().filter(|x| x.contains(pattern)).collect();
    items_match_pattern.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sorted_paths_test() {

    }
}
