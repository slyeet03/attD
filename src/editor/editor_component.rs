use super::Key;
use super::buffer::Buffer;
use super::cursor::Cursor;
use super::selection::Selection;
use gpui::*;
use gpui_component::*;

pub struct EditorComponent {
    cursor: Cursor,
    selection: Selection,
    scroll_offset: ScrollOffset,
}

pub struct ScrollOffset {
    width: usize,
    height: usize,
}

impl EditorComponent {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn handle_keypress(&mut self, key: Key, buffer: &mut Buffer) {
        // todo: add more keys
        match key {
            Key::Up => {
                self.cursor.move_up(buffer);
                self.selection.clear();
            }
            Key::Left => {
                self.cursor.move_left(buffer);
                self.selection.clear();
            }
            Key::Down => {
                self.cursor.move_down(buffer);
                self.selection.clear();
            }
            Key::Right => {
                self.cursor.move_right(buffer);
                self.selection.clear();
            }
            Key::Char(c) => {
                // todo: make it so when a selection is there that selection gets deleted and gets
                // replaced by the text
                let offset = self
                    .cursor
                    .to_offset(self.cursor.row, self.cursor.col, buffer);
                buffer.insert(offset, c);
            }
            Key::Backspace => {
                let offset = self
                    .cursor
                    .to_offset(self.cursor.row, self.cursor.col, buffer);
                self.cursor.col -= 1;
                buffer.delete_range(offset - 1, offset);
            }
            Key::Delete => {
                let offset = self
                    .cursor
                    .to_offset(self.cursor.row, self.cursor.col, buffer);
                buffer.delete_range(offset, offset + 1);
            }

            _ => {}
        }
    }
    pub fn handle_mouse_click(&mut self, row: usize, col: usize) {}
    pub fn handle_mouse_drag(&mut self, row: usize, col: usize) {}
}

impl Render for EditorComponent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child("editor bar")
    }
}
