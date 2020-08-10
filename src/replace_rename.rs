use std::path::{Path, PathBuf};
use std::borrow::Cow;

use crate::filesystem::{bulk_rename, get_string_paths};

use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use regex::Regex;

pub struct ReplaceRename<'a> {
    path_strs: Vec<String>,
    regex: Regex,
    replace: &'a str,
}

impl<'a> ReplaceRename<'a> {
    pub fn new(pattern: &str, replace: &'a str, allow_hidden: bool) -> Result<Self> {
        let path_strs = get_string_paths("./", allow_hidden)?;
        let regex = Regex::new(pattern)
            .context(format!("Failed to create regex with pattern {}", pattern))?;

        Ok(ReplaceRename {
            path_strs,
            regex,
            replace,
        })
    }

    /// Replaces all strings in `self.path_strs` with `self.replace` based on a regex. This
    /// function will return a new vector of path str containing the replace.
    pub fn replace(&self) -> Vec<Cow<'_, str>> {
        self.path_strs
            .par_iter()
            .map(|s| {
                self.regex.replace_all(&s, self.replace)
            })
            .collect()
    }

    pub fn rename_using_replace(&self, replaced: &[&str]) -> Result<()> {
        bulk_rename(&self.path_strs, replaced)?;

        Ok(())
    }
}
