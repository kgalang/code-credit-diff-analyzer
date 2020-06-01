use unidiff::{Line};
use std::io::prelude::*;
use std::fs::File;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::fmt;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Language {
    Other,
    Python,
    Ruby,
    Rust,
    Javascript,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

#[pymethods]
impl HunkStats {
    #[getter]
    fn lang(&self) -> PyResult<String> {
        Ok(self.lang.to_string())
    }

    #[getter]
    fn raw_added(&self) -> PyResult<usize> {
        Ok(self.raw_added)
    }

    #[getter]
    fn raw_removed(&self) -> PyResult<usize> {
        Ok(self.raw_removed)
    }

    #[getter]
    fn cleaned_added(&self) -> PyResult<usize> {
        Ok(self.cleaned_added)
    }

    #[getter]
    fn cleaned_removed(&self) -> PyResult<usize> {
        Ok(self.cleaned_removed)
    }
}

impl fmt::Display for HunkStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}