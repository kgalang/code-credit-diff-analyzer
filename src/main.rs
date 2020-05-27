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
    pub added_ct: i32,
    pub removed_ct: i32,
    pub trimmed_added_ct: i32,
    pub trimmed_removed_ct: i32,
}

#[derive(Debug)]
pub struct DiffStats(pub Vec<HunkStats>);

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
    
    for f in files {
        println!("f here ------- {:?}", f);
        for h in f.hunks() {
            get_hunk_stats(f, h);
        }
    }

    Ok(())
}
