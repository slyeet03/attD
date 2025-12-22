use crate::editor::editor_component::EditorComponent;

pub mod cursor;
pub mod delete;
pub mod selection;
pub mod text_input;
pub mod undoredo;

pub trait EditorCommand {
    fn execute(&self, editor: &mut EditorComponent);
    fn description(&self) -> &str;
}

pub use cursor::{Direction, MoveCursorCommand, MoveToLineEndCommand, MoveToLineStartCommand};
pub use delete::{DeleteBackwardCommand, DeleteForwardCommand};
pub use selection::SelectAllCommand;
pub use text_input::{InsertCharCommand, InsertNewlineCommand, InsertTabCommand};
pub use undoredo::{RedoCommand, UndoCommand};
