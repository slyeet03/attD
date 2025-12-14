use super::EditorCommand;
use crate::editor::editor_component::EditorComponent;

pub struct UndoCommand;

impl EditorCommand for UndoCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        println!("UNDO");
        editor.buffer.undo();
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Undo"
    }
}

pub struct RedoCommand;

impl EditorCommand for RedoCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        println!("REDO");
        editor.buffer.redo();
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Redo"
    }
}
