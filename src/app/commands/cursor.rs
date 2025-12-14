use super::EditorCommand;
use crate::editor::editor_component::EditorComponent;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MoveCursorCommand {
    pub direction: Direction,
    pub extend_selection: bool,
}

impl EditorCommand for MoveCursorCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        match self.direction {
            Direction::Up => editor.cursor.move_up(&editor.buffer),
            Direction::Down => editor.cursor.move_down(&editor.buffer),
            Direction::Left => editor.cursor.move_left(&editor.buffer),
            Direction::Right => editor.cursor.move_right(&editor.buffer),
        }

        if !self.extend_selection {
            editor.selection.clear();
        } else {
            editor
                .selection
                .expand_to(editor.cursor.row, editor.cursor.col);
        }

        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        match self.direction {
            Direction::Up => "Move cursor up",
            Direction::Down => "Move cursor down",
            Direction::Left => "Move cursor left",
            Direction::Right => "Move cursor right",
        }
    }
}

pub struct MoveToLineStartCommand {
    pub extend_selection: bool,
}

impl EditorCommand for MoveToLineStartCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        editor.cursor.move_to_line_start();

        if !self.extend_selection {
            editor.selection.clear();
        }

        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Move to line start"
    }
}

pub struct MoveToLineEndCommand {
    pub extend_selection: bool,
}

impl EditorCommand for MoveToLineEndCommand {
    fn execute(&self, editor: &mut EditorComponent) {
        editor.cursor.move_to_end(&editor.buffer);

        if !self.extend_selection {
            editor.selection.clear();
        }

        println!(
            "AFTER  - Cursor: row={}, col={}",
            editor.cursor.row, editor.cursor.col
        );
    }

    fn description(&self) -> &str {
        "Move to line end"
    }
}
