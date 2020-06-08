#[macro_use]
extern crate lazy_static;

mod analyzer;
mod models;

use analyzer::analyze_diff;
use models::{DiffString, HunkStats, Language};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_EXT: HashMap<&'static str, Language> = {
        let mut map = HashMap::new();
        map.insert("py", Language::Python);
        map.insert("rb", Language::Ruby);
        map.insert("rs", Language::Rust);
        map.insert("js", Language::Javascript);
        map
    };
    static ref LANG_COMMENTS: HashMap<Language, &'static str> = {
        let mut map = HashMap::new();
        map.insert(Language::Python, "#");
        map.insert(Language::Ruby, "#");
        map.insert(Language::Javascript, "//");
        map.insert(Language::Rust, "//");
        map
    };
    static ref RE_SOURCE_FILENAME: Regex = Regex::new(r"^--- (?P<filename>[^\t\n]+)(?:\t(?P<timestamp>[^\n]+))?").unwrap();
    static ref RE_TARGET_FILENAME: Regex = Regex::new(r"^\+\+\+ (?P<filename>[^\t\n]+)(?:\t(?P<timestamp>[^\n]+))?").unwrap();
    static ref RE_HUNK_HEADER: Regex = Regex::new(r"^@@ -(?P<source_start>\d+)(?:,(?P<source_length>\d+))? \+(?P<target_start>\d+)(?:,(?P<target_length>\d+))? @@[ ]?(?P<section_header>.*)").unwrap();
    static ref RE_HUNK_BODY_LINE: Regex = Regex::new(r"^(?P<line_type>[- \n\+\\]?)(?P<value>.*)").unwrap();
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn analyze(path_to_file: &str) -> PyResult<Vec<HunkStats>> {
    let diff = DiffString::from_file(path_to_file)?;

    let all_stats = analyze_diff(diff);

    Ok(all_stats)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn code_credit_diff(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(analyze))?;

    Ok(())
}
