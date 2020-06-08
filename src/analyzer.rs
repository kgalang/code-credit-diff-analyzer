use super::{LANG_COMMENTS, LANG_EXT};
use crate::diff::{Hunk, Line, PatchSet, PatchedFile};
use crate::models::{HunkStats, Language};
use std::path::Path;

fn is_significant_code(line: &Line, lang: &Language) -> bool {
    let trimmed_line = line.value.trim_start();
    // check for blank line
    if trimmed_line.is_empty() {
        return false;
    }

    // Check for comment
    if let Language::Other = lang {
        return true;
    }
    let comm_pre = LANG_COMMENTS.get(lang).expect("Not in map");
    let comm_pre_len = comm_pre.len();
    if trimmed_line.len() < comm_pre_len {
        return true;
    }

    &trimmed_line[..comm_pre_len] != *comm_pre
}

fn clean_hunk(_raw_hunk: Hunk, lang: &Language) -> Hunk {
    let mut cleaned = Hunk::new(0, 0, 0, 0, "");

    if let Language::Other = lang {
        return _raw_hunk;
    }

    // go through hunk source lines and remove comments here
    let cleaned_lines: Vec<&Line> = _raw_hunk
        .lines()
        .iter()
        .filter(|l| is_significant_code(*l, lang))
        .collect();

    for l in cleaned_lines {
        cleaned.append(l.clone());
    }

    cleaned
}

fn get_file_lang(file: &PatchedFile) -> &Language {
    let lang: &Language;
    let f_ext = Path::new(file.target_file.as_str()).extension();

    if let Some(ext) = f_ext {
        let ext_str = &ext.to_str().unwrap();

        if LANG_EXT.contains_key(ext_str) {
            lang = LANG_EXT.get(ext_str).expect("err getting language")
        } else {
            lang = &Language::Other;
        }
    } else {
        lang = &Language::Other;
    }

    lang
}

fn get_hunk_stats(_raw_hunk: &Hunk, cleaned_hunk: &Hunk, lang: &Language) -> HunkStats {
    HunkStats {
        lang: lang.clone(),
        raw_added: _raw_hunk.added(),
        raw_removed: _raw_hunk.removed(),
        raw_lines: _raw_hunk.lines().to_vec(),
        cleaned_added: cleaned_hunk.added(),
        cleaned_removed: cleaned_hunk.removed(),
        cleaned_lines: cleaned_hunk.lines().to_vec(),
    }
}

pub fn analyze_diff(diff: String) -> Vec<HunkStats> {
    let mut patch = PatchSet::new();
    patch.parse(diff).expect("Error parsing diff");
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

    all_stats
}
