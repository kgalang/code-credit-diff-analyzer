use crate::parser::models::*;
use regex::{Regex, Captures};

lazy_static! {
  static ref DIFF_FILE_START: Regex = Regex::new(r#"(^diff --git) "?(.+)"? "?(.+)"?"#).unwrap();
  static ref HUNK_HEADER: Regex = Regex::new(r"(^@@)").unwrap();
}

pub fn parse(diff: String) -> Vec<DiffFile>{
  // iterate through each line
  let mut out_files: Vec<DiffFile> = Vec::new();  
  let mut i = 0;

  for line in diff.lines() {
    i += 1;

    if let Some(caps) = DIFF_FILE_START.captures(line) {
      println!("{} DIFF START: {}", i, line)
    } else if let Some(caps) = HUNK_HEADER.captures(line) {
      println!("{} HUNK HEADER: {}", i, line)
    }
  }
  
  
  
  // placeholder
  let ex_file = DiffFile {
    old_name: "old.py".to_string(),
    new_name: "new.py".to_string(),
    mode: DiffFileModes::Modify,
    added_lines: 0,
    deleted_lines: 0,
    is_combined: false,
    lang: Languages::Python,
    hunks: None,
    hunk_header: None,
  };
  
  vec![ex_file]
}
      // fn get_files(full_diff: String) -> () {
      //   let mut count = 0;
      //   for l in full_diff.lines() {
      //     if l.contains("diff --git") {
      //       count += 1;
      //       println!("line {:?}", l)
      //     }
      //   }
      //   println!("count: {}", count);
      // }