#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use unidiff::{PatchSet, PatchedFile, Hunk, Line};
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Language {
    Other,
    Python,
    Ruby,
    Rust,
    Javascript,
}

lazy_static!{
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
    pub lang: Language,
    pub added: usize,
    pub removed: usize,
    pub cleaned_lines: Vec<Line>,
}

#[derive(Debug)]
pub struct DiffStats(pub Vec<HunkStats>);

fn is_not_comment(line: &Line, lang: &Language) -> bool {
    if let Language::Other = lang {
        return true
    }

    let trimmed_line = line.value.trim_start();
    let comm_pre = LANG_COMMENTS.get(lang).expect("Not in map");
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

fn clean_hunk(raw_hunk: Hunk, lang: &Language) -> Hunk{
    let mut cleaned = Hunk::new(0,0,0,0,"",);

    if let Language::Other = lang {
        return raw_hunk.clone()
    }

    // go through hunk source lines and remove comments here
    let cleaned_lines: Vec<&Line> = raw_hunk.lines().into_iter()
        .filter(|l| is_not_comment(*l, lang)).collect();

    for l in cleaned_lines {
        cleaned.append(l.clone());
    }

    cleaned
}

fn get_file_lang(file: &PatchedFile) -> &Language {
    let mut lang: &Language;
    let f_ext = Path::new(file.target_file.as_str())
        .extension();
    
    if f_ext.is_none() || !LANG_EXT.contains_key(f_ext.unwrap().to_str().unwrap()) {
        lang = &Language::Other;
    } else {
        lang = LANG_EXT.get(f_ext.unwrap().to_str().unwrap()).expect("err getting language")
    }

    lang
}

fn get_hunk_stats(raw_hunk: &Hunk, cleaned_hunk: &Hunk, lang: &Language) -> HunkStats {
    let stats = HunkStats{
        lang: lang.clone(),
        // mode: String,
        added: cleaned_hunk.added(),
        removed: cleaned_hunk.removed(),
        cleaned_lines: cleaned_hunk.lines().to_vec(),
    };
    stats
}

fn main() -> std::io::Result<()> {
    let diff = DiffString::from_file("./fixtures/rustball.diff")?;

    let mut patch = PatchSet::new();
    patch.parse(diff).ok().expect("Error parsing diff");
    let files = patch.files();
    let mut all_stats: Vec<HunkStats> = Vec::new();

    for f in files {
        let lang = get_file_lang(f);

        // loop through all hunks and output hunkstats for each
        for h in f.hunks() {
            let cleaned = clean_hunk(h.clone(), lang);
            let stats = get_hunk_stats(h, &cleaned, lang);
            all_stats.push(stats);
        }
    }

    println!("{}", all_stats.len());
    Ok(())
}
