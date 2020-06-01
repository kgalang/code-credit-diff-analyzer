use unidiff::{Line};
use std::io::prelude::*;
use std::fs::File;
use pyo3::prelude::*;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Language {
    Other,
    Python,
    Ruby,
    Rust,
    Javascript,
}

pub struct DiffString;

impl DiffString {
    pub fn from_file(path: &str) -> Result<String, std::io::Error> {
        let mut input = File::open(path)?;
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;
        Ok(input_buffer)
    }
    // fn from_github
}

#[pyclass]
#[derive(Debug)]
pub struct HunkStats {
    pub lang: Language,
    pub raw_added: usize,
    pub raw_removed: usize,
    pub raw_lines: Vec<Line>,
    pub cleaned_added: usize,
    pub cleaned_removed: usize,
    pub cleaned_lines: Vec<Line>,
}
