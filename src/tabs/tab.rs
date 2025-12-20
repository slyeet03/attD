use crate::editor::{Buffer, Cursor, Selection, editor_component::ScrollOffset, selection::Anchor};
use std::path::PathBuf;

pub struct Tab {
    pub id: usize,
    pub name: String,
    pub path: Option<PathBuf>,
    pub buffer: Buffer,
    pub dirty: bool,
    pub cursor: Cursor,
    pub selection: Selection,
    pub scroll_offset: ScrollOffset,
}

impl Tab {
    pub fn new_empty(id: usize) -> Self {
        Self {
            id,
            name: "Untitled".to_string(),
            path: None,
            buffer: Buffer::new(),
            dirty: false,
            cursor: Cursor { row: 0, col: 0 },
            selection: Selection {
                anchor: Anchor { row: 0, col: 0 },
                cursor: Cursor { row: 0, col: 0 },
            },
            scroll_offset: ScrollOffset {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn new_from_file(id: usize, path: PathBuf) -> Self {
        Self {
            id,
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: Some(path),
            buffer: Buffer::new(), //replace this with a from file fn
            dirty: false,
            cursor: Cursor { row: 0, col: 0 },
            selection: Selection {
                anchor: Anchor { row: 0, col: 0 },
                cursor: Cursor { row: 0, col: 0 },
            },
            scroll_offset: ScrollOffset {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn set_dirty(&mut self, flag: bool) {
        self.dirty = flag;
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }

    pub fn save_editor_state(
        &mut self,
        buffer_text: String,
        cursor_row: usize,
        cursor_col: usize,
        anchor_row: usize,
        anchor_col: usize,
        scroll_width: usize,
        scroll_height: usize,
        has_changes: bool,
    ) {
        self.buffer.text = buffer_text;
        self.cursor.row = cursor_row;
        self.cursor.col = cursor_col;
        self.selection.anchor.row = anchor_row;
        self.selection.anchor.col = anchor_col;
        self.scroll_offset.height = scroll_height;
        self.scroll_offset.width = scroll_width;
        self.dirty = has_changes;
        // new tab cannot have previous tab changes and shit
        self.buffer.undo_stack.clear();
        self.buffer.redo_stack.clear();
    }
    pub fn get_editor_state(&self) -> (String, usize, usize, usize, usize, usize, usize) {
        (
            self.buffer.text.clone(),
            self.cursor.row,
            self.cursor.col,
            self.selection.anchor.row,
            self.selection.anchor.col,
            self.scroll_offset.width,
            self.scroll_offset.height,
        )
    }
}
