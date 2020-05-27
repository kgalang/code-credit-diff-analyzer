use std::fs::File;
use std::io::prelude::*;
use unidiff::{PatchSet, PatchedFile, Hunk};
use std::path::Path;

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

fn clean_hunk(hunk: Hunk) -> Hunk{
    // go through hunk source lines and remove comments here
    hunk
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
        let cleaned: Vec<Hunk> = f.clone().into_iter()
            .map(|h| trim_hunk_lines(h)).collect();
        cleaned_hunks.extend(cleaned);
    }

    Ok(())
}
