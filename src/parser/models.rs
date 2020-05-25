pub enum DiffFileModes {
  Delete,
  New,
  Modify,
}

pub enum Languages {
  Python,
  Javascript,
  Go,
  Rust,
}

pub enum DiffLineActions {
  Insert,
  Delete,
  Context,
}

pub struct DiffFile {
  pub old_name: String,
  pub new_name: String,
  pub mode: DiffFileModes,
  pub added_lines: i32,
  pub deleted_lines: i32,
  pub is_combined: bool,
  pub lang: Languages,
  pub hunks: Option<Vec<DiffHunk>>
}

pub struct DiffHunk {
  pub header: String,
  pub lines: Vec<DiffLine>
}

pub struct DiffLine {
  action: DiffLineActions,
  content: String,
}