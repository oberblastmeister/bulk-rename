use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, io};

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;

pub fn get_string_paths(dir: impl AsRef<Path>, allow_hidden: bool) -> Result<Vec<String>> {
    let paths = get_sorted_paths(dir)?;
    Ok(convert_paths_to_string_iter(paths, allow_hidden))
}

/// get paths in dir specified and return unstably sorted vector
/// filter the paths that were returned successfully
fn get_sorted_paths(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(dir)?
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|res| res.map(|e| e.path()))
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    entries.sort_unstable();

    Ok(entries)
}

/// converts pathbufs into strings, filters for the ones that were converted successfully
fn convert_paths_to_string_iter(paths: Vec<PathBuf>, allow_hidden: bool) -> Vec<String> {
    let path_str_iter = paths
        .into_par_iter()
        .map(|p| {
            let is_dir = p.is_dir();
            p.into_os_string()
                .into_string()
                .map_err(|_| anyhow!("Could not convert OsString to a utf-8 String"))
                .map(|mut s| {
                    remove_front(&mut s);
                    if is_dir {
                        add_dir_slash(&mut s)
                    }
                    s
                })
        })
        .filter_map(|r| r.ok()); // only keep Ok values and also unwrap them

        if !allow_hidden {
            filter_hidden(path_str_iter).collect()
        } else {
            path_str_iter.collect()
        }
}

pub fn filter_hidden(
    iter: impl ParallelIterator<Item = String>,
) -> impl ParallelIterator<Item = String> {
    iter.filter(|s| !s.starts_with('.'))
}

fn is_hidden(s: &str) -> bool {
    s.starts_with('.')
}

/// removes ./ at the front
fn remove_front(s: &mut String) {
    if s.starts_with("./") {
        *s = s.chars().skip(2).collect()
    }
}

fn add_dir_slash(s: &mut String) {
    s.push('/')
}

/// renames from slices instead of single items
/// uses rayon to do it in parallel
/// this functions returns all the errors that occurred when renaming files
pub fn bulk_rename(from: &[String], to: &[&str]) -> Vec<anyhow::Error> {
    from.par_iter()
        .zip(to.par_iter())
        .map(|(f, t)| {
            if f != t {
                fs::rename(f, t).context(format!("Failed to rename {} to {}", f, t))
            } else {
                Ok(())
            }
        })
        .filter_map(|r| r.err()) // only keep error values and unwrap_err them
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn testing_strs() -> Vec<&'static str> {
        vec!["file1", "file2", "file3", "file4", "file5", "hello_dude"]
    }

    fn testing_strs_dot() -> Vec<&'static str> {
        vec![
            "./file1",
            "./file2",
            "./file3",
            "./file4",
            "./file5",
            "./hello_dude",
        ]
    }

    fn testing_pathbufs_from_strs(paths: Vec<&str>) -> Vec<PathBuf> {
        paths.iter().map(|p| PathBuf::from(p)).collect()
    }

    #[test]
    fn get_sorted_paths_test() -> Result<()> {
        env::set_current_dir("tests/get_sorted_paths_test")?;

        let paths = get_sorted_paths(".")?;
        assert_eq!(paths, testing_pathbufs_from_strs(testing_strs_dot()));

        Ok(())
    }

    #[test]
    fn remove_front_test() {
        assert_eq!(remove_front(String::from("./intersting")), "intersting");
    }

    #[test]
    fn remove_front_test_no_dot() {
        assert_eq!(remove_front(String::from("wow")), "wow");
    }

    #[test]
    fn remove_front_test_wrong_place() {
        assert_eq!(remove_front(String::from("wrong./place")), "wrong./place");
    }

    #[test]
    fn remove_front_test_end() {
        assert_eq!(remove_front(String::from("attheend./")), "attheend./");
    }

    #[test]
    fn convert_test() -> Result<()> {
        let paths = testing_pathbufs_from_strs(testing_strs_dot());
        let items = convert_paths_to_string_iter(paths).collect::<Result<Vec<String>>>()?;

        assert_eq!(items, testing_strs());

        Ok(())
    }
}
