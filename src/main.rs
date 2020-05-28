#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use unidiff::{PatchSet, PatchedFile, Hunk, Line};
use std::path::Path;
use std::collections::HashMap;

lazy_static!{
    static ref LANG_COMMENTS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("py", "#");
        map.insert("rb", "#");
        map.insert("js", "//");
        map.insert("rs", "//");
        map
    };
}

struct DiffString;

impl DiffString {
    fn from_file(path: &str) -> Result<String, std::io::Error> {
        let mut input = File::open(path)?;
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;
        Ok(input_buffer)
    }
    // fn from_github
}

#[derive(Debug)]
pub struct HunkStats {
    pub lang: String,
    pub mode: String,
    pub added: i32,
    pub removed: i32,
    pub cleaned_source: Vec<String>,
}

#[derive(Debug)]
pub struct DiffStats(pub Vec<HunkStats>);

fn is_not_comment(line: &Line, f_ext: Option<&str>) -> bool {
    if f_ext.is_none() || !LANG_COMMENTS.contains_key(f_ext.unwrap()){
        return true
    }

    let trimmed_line = line.value.trim_start();
    let comm_pre = LANG_COMMENTS.get(f_ext.unwrap()).expect("Not in map");
    let comm_pre_len = comm_pre.len();
    if trimmed_line.len() < comm_pre_len {
        return true
    }

    if &trimmed_line[..comm_pre_len] == *comm_pre {
        return false
    } else {
        return true
    }
}

fn clean_hunk(hunk: Hunk, f_ext: Option<&str>) -> Hunk{
    // go through hunk source lines and remove comments here
    let cleaned_lines: Vec<&Line> = hunk.lines().into_iter()
        .filter(|l| is_not_comment(*l, f_ext)).collect();

    // print cleaned lines
    for l in cleaned_lines {
        println!("{:?}", l.value);
    }

    hunk
}

fn get_file_ext(file: &PatchedFile) -> Option<&str> {
    let f_ext = Path::new(file.target_file.as_str())
        .extension().expect("error getting extension");
    f_ext.to_str()
}

fn get_hunk_stats(file: &PatchedFile, hunk: &Hunk) {
    let f_ext = Path::new(file.target_file.as_str()).extension().unwrap();
    println!("{:?}", f_ext);
}

fn main() -> std::io::Result<()> {
    let diff = DiffString::from_file("./fixtures/rustball.diff")?;

    let mut patch = PatchSet::new();
    patch.parse(diff).ok().expect("Error parsing diff");
    let files = patch.files();
    let mut out_stats: Vec<HunkStats> = Vec::new();
    let mut cleaned_hunks: Vec<Hunk> = Vec::new();
    
    // trim hunks to remove comments from counting for added or removed properties
    for f in files {
        let f_ext = get_file_ext(f);
        let cleaned: Vec<Hunk> = f.clone().into_iter()
            .map(|h| clean_hunk(h, f_ext)).collect();
        cleaned_hunks.extend(cleaned);
    }

    Ok(())
}
