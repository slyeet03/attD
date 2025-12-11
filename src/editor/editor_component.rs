use super::Key;
use super::buffer::Buffer;
use super::cursor::Cursor;
use super::selection::Selection;
use crate::editor::selection::Anchor;
use gpui::AbsoluteLength;

use gpui::*;
use gpui_component::text::Text;
use gpui_component::text::TextViewStyle;
use gpui_component::*;

pub struct EditorComponent {
    cursor: Cursor,
    selection: Selection,
    scroll_offset: ScrollOffset,
    buffer: Buffer,
}

pub struct ScrollOffset {
    width: usize,
    height: usize,
}

impl EditorComponent {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            cursor: Cursor { row: 0, col: 0 },
            selection: Selection {
                anchor: Anchor { row: 0, col: 0 },
                cursor: Cursor { row: 0, col: 0 },
            },
            scroll_offset: ScrollOffset {
                width: 0,
                height: 0,
            },
            buffer: Buffer::new(),
        }
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
                self.buffer.insert(offset, c);
            }
            // todo: selection leke bhi delete krna hain multiple lines for backspace and delete
            Key::Backspace => {
                let offset = self
                    .cursor
                    .to_offset(self.cursor.row, self.cursor.col, buffer);
                self.cursor.col -= 1;
                self.buffer.delete_range(offset - 1, offset);
            }
            Key::Delete => {
                let offset = self
                    .cursor
                    .to_offset(self.cursor.row, self.cursor.col, buffer);
                self.buffer.delete_range(offset, offset + 1);
            }

            _ => {}
        }
    }
    pub fn handle_mouse_click(&mut self, mut row: usize, mut col: usize) {
        row = row.min(self.buffer.line_count() - 1);
        col = col.min(
            self.buffer
                .get_line(row)
                .map(|line| line.len())
                .unwrap_or(0),
        );

        self.cursor.row = row;
        self.cursor.col = col;
        self.selection.clear();
    }
    pub fn handle_mouse_drag(&mut self, mut row: usize, mut col: usize) {
        row = row.min(self.buffer.line_count() - 1);
        col = col.min(
            self.buffer
                .get_line(row)
                .map(|line| line.len())
                .unwrap_or(0),
        );
        self.selection.expand_to(row, col);
    }
}

impl Render for EditorComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let lines = self.buffer.as_lines();
        let (sr, sc, er, ec) = self.selection.range();

        // Character size in pixels
        let char_width = 8.0;
        let line_height = 16.0;

        // Render all lines with selection highlight
        let text_lines: Vec<_> = lines
            .iter()
            .enumerate()
            .map(|(row, line)| {
                if row >= sr && row <= er {
                    let start = if row == sr { sc } else { 0 };
                    let end = if row == er { ec } else { line.len() };

                    let before = &line[..start.min(line.len())];
                    let mid = &line[start.min(line.len())..end.min(line.len())];
                    let after = &line[end.min(line.len())..];

                    div()
                        .flex_row()
                        .child(Text::from(before))
                        .child(
                            div() // Highlighted part
                                .child(Text::from(mid))
                                .bg(rgb(0x99bbff)),
                        )
                        .child(Text::from(after))
                } else {
                    div().flex_row().child(Text::from(line.clone()))
                }
            })
            .collect();

        // Cursor position as absolute div
        let cursor_div = div()
            .absolute()
            .left(AbsoluteLength::Pixels(
                (self.cursor.col as f32 * char_width).into(),
            ))
            .top(AbsoluteLength::Pixels(
                (self.cursor.row as f32 * line_height).into(),
            ))
            .w(AbsoluteLength::Pixels(2.0.into()))
            .h(AbsoluteLength::Pixels(line_height.into()))
            .bg(rgb(0x0000ff)); // bright blue cursor

        // Combine text lines and cursor
        div().flex_col().children(text_lines).child(cursor_div)
    }
}
