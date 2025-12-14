use super::EditorCommand;
use crate::editor::editor_component::EditorComponent;
use crate::editor::selection::Anchor;

pub struct SelectAllCommand;

impl EditorCommand for SelectAllCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        println!("SELECT ALL");
        editor.cursor.row = 0;
        editor.cursor.col = 0;
        editor.selection.anchor = Anchor { row: 0, col: 0 };

        let last_line = editor.buffer.line_count().saturating_sub(1);
        let last_col = editor
            .buffer
            .get_line(last_line)
            .map(|l| l.len())
            .unwrap_or(0);

        editor.selection.expand_to(last_line, last_col);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Select all"
    }
}
