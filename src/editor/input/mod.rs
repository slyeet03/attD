pub mod keyboard;
pub mod mouse;
use crate::editor::editor_component::EditorComponent;

pub fn pixel_to_position(editor_comp: &mut EditorComponent, x: f32, y: f32) -> (usize, usize) {
    let row = (y / editor_comp.line_height) as usize;
    let col = (x / editor_comp.char_width) as usize;

    let row = row.min(editor_comp.buffer.line_count().saturating_sub(1));
    let col = col.min(
        editor_comp
            .buffer
            .get_line(row)
            .map(|line| line.len())
            .unwrap_or(0),
    );

    (row, col)
}

pub fn insert_char(editor_comp: &mut EditorComponent, c: char) {
    editor_comp.dirty_flag = true;

    if !editor_comp.selection.is_empty() {
        delete_selection(editor_comp);
    }

    let offset = editor_comp.cursor.to_offset(
        editor_comp.cursor.row,
        editor_comp.cursor.col,
        &editor_comp.buffer,
    );
    editor_comp.buffer.insert(offset, c);
    editor_comp.cursor.move_right(&editor_comp.buffer);
}

pub fn delete_selection(editor_comp: &mut EditorComponent) {
    editor_comp.dirty_flag = true;

    let (sr, sc, er, ec) = editor_comp.selection.range();
    let start_offset = editor_comp.cursor.to_offset(sr, sc, &editor_comp.buffer);
    let end_offset = editor_comp.cursor.to_offset(er, ec, &editor_comp.buffer);

    editor_comp.buffer.delete_range(start_offset, end_offset);
    editor_comp.cursor.row = sr;
    editor_comp.cursor.col = sc;
    editor_comp.selection.clear();
}

pub fn handle_backspace(editor_comp: &mut EditorComponent) {
    editor_comp.dirty_flag = true;

    if !editor_comp.selection.is_empty() {
        delete_selection(editor_comp);
    } else if editor_comp.cursor.col > 0 || editor_comp.cursor.row > 0 {
        let offset = editor_comp.cursor.to_offset(
            editor_comp.cursor.row,
            editor_comp.cursor.col,
            &editor_comp.buffer,
        );
        if offset > 0 {
            editor_comp.buffer.delete_range(offset - 1, offset);
            editor_comp.cursor.move_left(&editor_comp.buffer);
        }
    }
}

pub fn handle_delete(editor_comp: &mut EditorComponent) {
    editor_comp.dirty_flag = true;

    if !editor_comp.selection.is_empty() {
        delete_selection(editor_comp);
    } else {
        let offset = editor_comp.cursor.to_offset(
            editor_comp.cursor.row,
            editor_comp.cursor.col,
            &editor_comp.buffer,
        );
        let max_offset = editor_comp.buffer.as_lines().join("\n").len();
        if offset < max_offset {
            editor_comp.buffer.delete_range(offset, offset + 1);
        }
    }
}

pub fn handle_enter(editor_comp: &mut EditorComponent) {
    editor_comp.dirty_flag = true;

    if !editor_comp.selection.is_empty() {
        delete_selection(editor_comp);
    }

    let offset = editor_comp.cursor.to_offset(
        editor_comp.cursor.row,
        editor_comp.cursor.col,
        &editor_comp.buffer,
    );
    editor_comp.buffer.insert(offset, '\n');
    editor_comp.cursor.row += 1;
    editor_comp.cursor.col = 0;
}

pub fn handle_tab(editor_comp: &mut EditorComponent) {
    for _ in 0..4 {
        insert_char(editor_comp, ' ');
    }
}
