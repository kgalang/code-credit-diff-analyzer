#[macro_use]
extern crate lazy_static;

mod models;
mod analyzer;

use std::collections::HashMap;
use models::{Language, DiffString};
use analyzer::{analyze_diff};

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

fn main() -> std::io::Result<()> {
    let diff = DiffString::from_file("./fixtures/rustball.diff")?;

    let all_stats = analyze_diff(diff);

    for s in all_stats {
        println!("hunk");
        println!("added: {} -> {}", s.raw_added, s.cleaned_added);
        println!("removed: {} -> {}", s.raw_removed, s.cleaned_removed);
    }

    Ok(())
}
