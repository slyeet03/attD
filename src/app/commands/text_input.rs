use super::EditorCommand;
use crate::editor::editor_component::EditorComponent;

pub struct InsertCharCommand {
    pub character: char,
}

impl EditorCommand for InsertCharCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        crate::editor::input::insert_char(editor, self.character);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Insert character"
    }
}

pub struct InsertNewlineCommand;

impl EditorCommand for InsertNewlineCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        crate::editor::input::handle_enter(editor);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Insert newline"
    }
}

pub struct InsertTabCommand;

impl EditorCommand for InsertTabCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        crate::editor::input::handle_tab(editor);
        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Insert tab (4 spaces)"
    }
}
