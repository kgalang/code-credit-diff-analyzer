use crate::parser::models::*;

pub fn parse(diff: String) -> Vec<DiffFile>{
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