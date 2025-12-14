use super::pixel_to_position;
use crate::editor::editor_component::EditorComponent;
use crate::editor::selection::Anchor;
use gpui::{MouseDownEvent, MouseMoveEvent, MouseUpEvent, Window};

pub struct MouseHandler;

impl MouseHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_mouse_down_static(
        editor: &mut EditorComponent,
        event: &MouseDownEvent,
        window: &mut Window,
    ) {
        println!("MOUSE DOWN EVENT FIRED!");
        println!(
            "MOUSE DOWN at x={}, y={}",
            event.position.x, event.position.y
        );

        let focus_handle = editor.focus_handle.clone();
        window.focus(&focus_handle);
        println!("Editor focused via mouse!");

        let (row, col) =
            pixel_to_position(editor, event.position.x.into(), event.position.y.into());

        println!("Clicked position: row={}, col={}", row, col);

        editor.cursor.row = row;
        editor.cursor.col = col;
        editor.selection.clear();
        editor.selection.anchor = Anchor { row, col };
        editor.is_dragging = true;
    }

    pub fn handle_mouse_move_static(editor: &mut EditorComponent, event: &MouseMoveEvent) {
        if !editor.is_dragging {
            return;
        }

        println!(
            "MOUSE DRAG at x={}, y={}",
            event.position.x, event.position.y
        );

        let (row, col) =
            pixel_to_position(editor, event.position.x.into(), event.position.y.into());

        editor.cursor.row = row;
        editor.cursor.col = col;
        editor.selection.expand_to(row, col);
    }

    pub fn handle_mouse_up_static(editor: &mut EditorComponent, _event: &MouseUpEvent) {
        println!("MOUSE UP - drag ended");
        editor.is_dragging = false;
    }
}

