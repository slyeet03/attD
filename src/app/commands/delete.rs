use super::EditorCommand;
use crate::editor::editor_component::EditorComponent;

pub struct DeleteBackwardCommand;

impl EditorCommand for DeleteBackwardCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        crate::editor::input::handle_backspace(editor);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Delete backward"
    }
}

pub struct DeleteForwardCommand;

impl EditorCommand for DeleteForwardCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        crate::editor::input::handle_delete(editor);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Delete forward"
    }
}
