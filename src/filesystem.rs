use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, io};

use anyhow::{anyhow, Context, Result, bail};
use rayon::prelude::*;

use crate::errors::combine_errors;

/// Gets a string representation of the paths in the specified directory.
pub fn get_string_paths(dir: impl AsRef<Path>, allow_hidden: bool) -> Result<Vec<String>> {
    let paths = get_sorted_paths(dir)?;
    Ok(convert_paths_to_string_iter(paths, allow_hidden))
}

/// Get paths in directory specified and return unstably sorted vector.
/// This function ignore any errors that occured and will print them to stderr.
fn get_sorted_paths(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(dir)?
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|res| res.map(|e| e.path()))
        .inspect(|res| {
            if let Some(e) = res.as_ref().err() {
                eprintln!("[bulk-rename error]: failed to read an entry, {:?}", e);
            }
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    entries.sort_unstable();

    Ok(entries)
}

/// Converts PathBufs into Strings. This function applies some additional niceties like removing
/// the ./ in front of the path and adding / at the end of a directory. This function also ignores
/// errors and will print the to stderr.
fn convert_paths_to_string_iter(paths: Vec<PathBuf>, allow_hidden: bool) -> Vec<String> {
    let path_str_iter = paths
        .into_par_iter()
        .map(|p| {
            let is_dir = p.is_dir();
            let res = p.into_os_string().into_string();

            // format string
            res.map(|mut s| {
                remove_front(&mut s);
                if is_dir {
                    add_dir_slash(&mut s)
                }
                s
            })
        })
        // print any errors that have happened
        .inspect(|res| {
            if let Some(e) = res.as_ref().err() {
                eprintln!(
                    "[bulk-rename error]: Could not convert OsString to a uft-8 String.
The OsString was {:?}",
                    e
                );
            }
        })
        // then discard the errors
        .filter_map(Result::ok);

    // collect into vec with hidden paths or not
    if !allow_hidden {
        filter_hidden(path_str_iter).collect()
    } else {
        path_str_iter.collect()
    }
}

fn filter_hidden(
    iter: impl ParallelIterator<Item = String>,
) -> impl ParallelIterator<Item = String> {
    iter.filter(|s| !s.starts_with('.'))
}

fn remove_front(s: &mut String) {
    if s.starts_with("./") {
        *s = s.chars().skip(2).collect()
    }
}

fn add_dir_slash(s: &mut String) {
    s.push('/')
}

/// Renames from slices instead of single items like `std::fs::rename`. This function uses rayon to
/// rename in parallel. This functions returns a vector of all the errors that have occurred.
pub fn bulk_rename(from: &[String], to: &[&str]) -> Result<()> {
    let errors: Vec<anyhow::Error> = from.par_iter()
        .zip(to.par_iter())
        .map(|(f, t)| {
            if f != t {
                fs::rename(f, t).context(format!("Failed to rename {} to {}", f, t))
            } else {
                Ok(())
            }
        })
        .filter_map(Result::err) // only keep error values and unwrap_err them
        .collect();

    if !errors.is_empty() {
        bail!(combine_errors(errors))
    } else {
        Ok(())
    }
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
