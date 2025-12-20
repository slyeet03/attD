use super::buffer::Buffer;
use super::cursor::Cursor;
use super::input::keyboard::KeyboardHandler;
use super::input::mouse::MouseHandler;
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
    pub focus_handle: FocusHandle,
    pub keyboard_handler: KeyboardHandler,
    pub mouse_handler: MouseHandler,
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
            keyboard_handler: KeyboardHandler::new(),
            mouse_handler: MouseHandler::new(),
        }
    }

    fn render_text_lines(&self) -> Vec<impl IntoElement> {
        let lines = self.buffer.as_lines();
        let (sr, sc, er, ec) = self.selection.range();

        lines
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
                        .child(div().bg(rgb(0x4a9eff)).child(Text::from(mid)))
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
            .collect()
    }

    fn render_cursor(&self) -> impl IntoElement {
        div()
            .absolute()
            .left(px(self.cursor.col as f32 * self.char_width))
            .top(px(self.cursor.row as f32 * self.line_height))
            .w(px(2.0))
            .h(px(self.line_height))
            .bg(rgb(0xffffff))
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

        window.focus(&self.focus_handle);
        println!("Focus set on render!");

        let text_lines = self.render_text_lines();
        let cursor = self.render_cursor();
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
                crate::editor::log::log_keystrokes(editor, event);

                if let Some(command) = editor.keyboard_handler.handle_key_event(editor, event) {
                    command.execute(editor);
                    println!("Buffer content: {:?}", editor.buffer.as_lines());
                    cx.notify();
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|editor, event: &MouseDownEvent, window, cx| {
                    MouseHandler::handle_mouse_down_static(editor, event, window);
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|editor, event: &MouseMoveEvent, _window, cx| {
                MouseHandler::handle_mouse_move_static(editor, event);
                if editor.is_dragging {
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|editor, event: &MouseUpEvent, _window, cx| {
                    MouseHandler::handle_mouse_up_static(editor, event);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .relative()
                    .children(text_lines)
                    .child(cursor),
            )
    }
}
