use super::buffer::Buffer;
use super::cursor::Cursor;
use super::input;
use super::selection::{Anchor, Selection};
use gpui::*;
use gpui_component::text::Text;
use gpui_component::*;

pub struct EditorComponent {
    pub cursor: Cursor,
    pub selection: Selection,
    pub scroll_offset: ScrollOffset,
    pub buffer: Buffer,
    pub char_width: f32,
    pub line_height: f32,
    pub is_dragging: bool,
    focus_handle: FocusHandle,
}

pub struct ScrollOffset {
    pub width: usize,
    pub height: usize,
}

impl EditorComponent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        println!("EditorComponent::new() called!");
        
        let mut buffer = Buffer::new();
        buffer.insert_str(0, "");
        
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
            buffer,
            char_width: 8.0,
            line_height: 20.0,
            is_dragging: false,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for EditorComponent {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for EditorComponent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("EditorComponent::render() called!");
        
        // Focus the editor immediately on first render
        window.focus(&self.focus_handle);
        println!("Focus set on render!");
        
        let lines = self.buffer.as_lines();
        let (sr, sc, er, ec) = self.selection.range();

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
                        .h(px(self.line_height))
                        .flex()
                        .flex_row()
                        .items_center()
                        .child(Text::from(before))
                        .child(
                            div()
                                .bg(rgb(0x4a9eff))
                                .child(Text::from(mid))
                        )
                        .child(Text::from(after))
                } else {
                    div()
                        .h(px(self.line_height))
                        .flex()
                        .flex_row()
                        .items_center()
                        .child(Text::from(line.clone()))
                }
            })
            .collect();

        let focus_handle = self.focus_handle.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .font_family("monospace")
            .text_size(px(14.0))
            .p_2()
            .track_focus(&focus_handle)
            .on_key_down(cx.listener(|editor, event: &KeyDownEvent, _window, cx| {
                println!("KEY DOWN EVENT FIRED!");
                println!("KEY PRESSED: '{}' | Modifiers: ctrl={} alt={} shift={} platform={}", 
                    event.keystroke.key, 
                    event.keystroke.modifiers.control,
                    event.keystroke.modifiers.alt,
                    event.keystroke.modifiers.shift,
                    event.keystroke.modifiers.platform
                );
                println!("BEFORE - Cursor: row={}, col={} | Buffer lines: {}", 
                    editor.cursor.row, 
                    editor.cursor.col,
                    editor.buffer.line_count()
                );
                
                match event.keystroke.key.as_str() {
                    "space" => {
                        println!("SPACE pressed");
                        input::insert_char(editor, ' ');
                        cx.notify();
                        return;

                    }
                    "up" => {
                        println!("UP arrow pressed");
                        editor.cursor.move_up(&editor.buffer);
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        } else {
                            editor.selection.expand_to(editor.cursor.row, editor.cursor.col);
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "down" => {
                        println!("DOWN arrow pressed");
                        editor.cursor.move_down(&editor.buffer);
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        } else {
                            editor.selection.expand_to(editor.cursor.row, editor.cursor.col);
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "left" => {
                        println!("LEFT arrow pressed");
                        editor.cursor.move_left(&editor.buffer);
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        } else {
                            editor.selection.expand_to(editor.cursor.row, editor.cursor.col);
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "right" => {
                        println!("RIGHT arrow pressed");
                        editor.cursor.move_right(&editor.buffer);
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        } else {
                            editor.selection.expand_to(editor.cursor.row, editor.cursor.col);
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "backspace" => {
                        println!("BACKSPACE pressed");
                        input::handle_backspace(editor);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "delete" => {
                        println!("DELETE pressed");
                        input::handle_delete(editor);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "enter" => {
                        println!("ENTER pressed");
                        input::handle_enter(editor);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "tab" => {
                        println!("⭾ TAB pressed");
                        input::handle_tab(editor);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "home" => {
                        println!("HOME pressed");
                        editor.cursor.move_to_line_start();
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "end" => {
                        println!("END pressed");
                        editor.cursor.move_to_end(&editor.buffer);
                        if !event.keystroke.modifiers.shift {
                            editor.selection.clear();
                        }
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "z" if event.keystroke.modifiers.platform => {
                        println!("UNDO");
                        editor.buffer.undo();
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "y" if event.keystroke.modifiers.platform => {
                        println!("REDO");
                        editor.buffer.redo();
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    "a" if event.keystroke.modifiers.platform => {
                        println!("SELECT ALL");
                        editor.cursor.row = 0;
                        editor.cursor.col = 0;
                        editor.selection.anchor = Anchor { row: 0, col: 0 };
                        let last_line = editor.buffer.line_count().saturating_sub(1);
                        let last_col = editor.buffer.get_line(last_line).map(|l| l.len()).unwrap_or(0);
                        editor.selection.expand_to(last_line, last_col);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        cx.notify();
                        return;
                    }
                    _ => {}
                }

                if event.keystroke.key.len() == 1
                    && !event.keystroke.modifiers.control
                    && !event.keystroke.modifiers.alt
                    && !event.keystroke.modifiers.platform
                {
                    if let Some(c) = event.keystroke.key.chars().next() {
                        println!("CHAR INPUT: '{}' (code: {})", c, c as u32);
                        input::insert_char(editor, c);
                        println!("AFTER  - Cursor: row={}, col={}", editor.cursor.row, editor.cursor.col);
                        println!("Buffer content: {:?}", editor.buffer.as_lines());
                        cx.notify();
                    }
                } else {
                    println!("Key not handled: '{}'", event.keystroke.key);
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|editor, event: &MouseDownEvent, window, cx| {
                    println!("MOUSE DOWN EVENT FIRED!");
                    println!("MOUSE DOWN at x={}, y={}", event.position.x, event.position.y);
                    window.focus(&editor.focus_handle);
                    println!("Editor focused via mouse!");
                    let (row, col) = input::pixel_to_position(
                        editor,
                        event.position.x.into(),
                        event.position.y.into(),
                    );
                    println!("Clicked position: row={}, col={}", row, col);
                    editor.cursor.row = row;
                    editor.cursor.col = col;
                    editor.selection.clear();
                    editor.selection.anchor = Anchor { row, col };
                    editor.is_dragging = true;
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|editor, event: &MouseMoveEvent, _window, cx| {
                if editor.is_dragging {
                    println!("MOUSE DRAG at x={}, y={}", event.position.x, event.position.y);
                    let (row, col) = input::pixel_to_position(
                        editor,
                        event.position.x.into(),
                        event.position.y.into(),
                    );
                    editor.cursor.row = row;
                    editor.cursor.col = col;
                    editor.selection.expand_to(row, col);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|editor, _event: &MouseUpEvent, _window, cx| {
                    println!("MOUSE UP - drag ended");
                    editor.is_dragging = false;
                    cx.notify();
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .relative()
                    .children(text_lines)
                    .child(
                        div()
                            .absolute()
                            .left(px(self.cursor.col as f32 * self.char_width))
                            .top(px(self.cursor.row as f32 * self.line_height))
                            .w(px(2.0))
                            .h(px(self.line_height))
                            .bg(rgb(0xffffff))
                    )
            )
    }
}
