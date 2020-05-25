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
  pub hunk_header: Option<String>,
}

#[derive(Debug)]
pub struct DiffHunk {
  pub header: String,
  pub lines: Vec<DiffLine>
}

#[derive(Debug)]
pub struct DiffLine {
  action: DiffLineActions,
  content: String,
}
