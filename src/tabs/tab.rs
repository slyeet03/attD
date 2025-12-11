use crate::editor::Buffer;
use std::path::PathBuf;

pub struct Tab {
    pub id: usize,
    pub name: String,
    pub path: Option<PathBuf>,
    pub buffer: Buffer,
    pub dirty: bool,
}

impl Tab {
    pub fn new_empty(id: usize) -> Self {
        Self {
            id,
            name: "Untitled".to_string(),
            path: None,
            buffer: Buffer::new(),
            dirty: false,
        }
    }

    pub fn new_from_file(id: usize, path: PathBuf) -> Self {
        Self {
            id,
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: Some(path),
            buffer: Buffer::new(), //replace this with a from file fn
            dirty: false,
        }
    }

    pub fn set_dirty(&mut self, flag: bool) {
        self.dirty = flag;
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }
}
