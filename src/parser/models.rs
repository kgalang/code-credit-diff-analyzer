enum DiffFileModes {
  Delete,
  New,
  Modify,
}

enum Languages {
  Python,
  Javascript,
  Go,
  Rust,
}

enum DiffLineActions {
  Insert,
  Delete,
  Context,
}

pub struct DiffFile {
  old_name: String,
  new_name: String,
  mode: DiffFileModes,
  added_lines: i32,
  deleted_lines: i32,
  is_combined: bool,
  lang: Languages,
  hunks: Vec<DiffHunk>
}

pub struct DiffHunk {
  header: String,
  lines: Vec<DiffLine>
}

pub struct DiffLine {
  action: DiffLineActions,
  content: String,
}