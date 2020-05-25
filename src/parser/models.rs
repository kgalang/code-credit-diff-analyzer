#[derive(Debug)]
pub enum DiffFileModes {
  Delete,
  New,
  Modify,
}

#[derive(Debug)]
pub enum Languages {
  Python,
  Javascript,
  Go,
  Rust,
}

#[derive(Debug)]
pub enum DiffLineActions {
  Insert,
  Delete,
  Context,
}

#[derive(Debug)]
pub struct DiffFile {
  pub old_name: String,
  pub new_name: String,
  pub mode: DiffFileModes,
  pub added_lines: i32,
  pub deleted_lines: i32,
  pub is_combined: bool,
  pub lang: Languages,
  pub hunks: Option<Vec<DiffHunk>>,
}

#[derive(Debug)]
pub struct DiffHunk {
  pub header: String,
  pub lines: Vec<DiffLine>
}

impl DiffHunk {
  fn new(header: String) -> DiffHunk {
    DiffHunk {
      header: header,
      lines: Vec::new(),
    }
  }
}

#[derive(Debug)]
pub struct DiffLine {
  pub action: DiffLineActions,
  pub content: String,
}
