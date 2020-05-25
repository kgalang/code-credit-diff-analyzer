use crate::parser::models::*;
use regex::{Regex, Captures};

lazy_static! {
  static ref DIFF_FILE_START: Regex = Regex::new(r#"(^diff --git) "?(.+)"? "?(.+)"?"#).unwrap();
  static ref HUNK_HEADER: Regex = Regex::new(r"(^@@)").unwrap();
}

pub fn create_line(line: String) -> DiffLine {
  // line actions
  let insert_prefix = "+";
  let delete_prefix = "-";

  let mut action = DiffLineActions::Context;
  if line.starts_with(insert_prefix) {
    action = DiffLineActions::Insert;
  } else if line.starts_with(delete_prefix) {
    action = DiffLineActions::Delete
  }

  DiffLine {
    action: action,
    content: line
  }
}

pub fn parse(diff: String) -> Vec<DiffFile>{
  // iterate through each line
  // if file start
  //    save previous file to out_files vec and create new struct file
  // if hunk header
  //    add to DiffFile hunk_header
  //    save previous hunk to cur file, then start new hunk
  // else
  //    create line and add to hunk

  let mut out_files: Vec<DiffFile> = Vec::new();  
  let mut i = 0;
  let mut lines = diff.lines();

  for line in lines {
    i += 1;

    if let Some(caps) = DIFF_FILE_START.captures(line) {
      println!("{} DIFF START: {}", i, line);
      // println!("DIFF START + 1: {}", line.next());
    } else if let Some(caps) = HUNK_HEADER.captures(line) {
      println!("{} HUNK HEADER: {}", i, line)
    } else {
      let diff_line = create_line(line.to_string());
      println!("{} {:?}", i, diff_line)
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