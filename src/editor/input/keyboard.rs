use crate::app::commands::{
    DeleteBackwardCommand, DeleteForwardCommand, Direction, EditorCommand, InsertCharCommand,
    InsertNewlineCommand, InsertTabCommand, MoveCursorCommand, MoveToLineEndCommand,
    MoveToLineStartCommand, RedoCommand, SelectAllCommand, UndoCommand,
};
use crate::editor::editor_component::EditorComponent;
use gpui::KeyDownEvent;

pub struct KeyboardHandler;

impl KeyboardHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_key_event(
        &self,
        editor: &EditorComponent,
        event: &KeyDownEvent,
    ) -> Option<Box<dyn EditorCommand>> {
        let key = event.keystroke.key.as_str();
        let modifiers = &event.keystroke.modifiers;

        if modifiers.platform {
            return match key {
                "z" => Some(Box::new(UndoCommand)),
                "y" => Some(Box::new(RedoCommand)),
                "a" => Some(Box::new(SelectAllCommand)),
                _ => None,
            };
        }

        let extend_selection = modifiers.shift;

        match key {
            "up" => Some(Box::new(MoveCursorCommand {
                direction: Direction::Up,
                extend_selection,
            })),
            "down" => Some(Box::new(MoveCursorCommand {
                direction: Direction::Down,
                extend_selection,
            })),
            "left" => Some(Box::new(MoveCursorCommand {
                direction: Direction::Left,
                extend_selection,
            })),
            "right" => Some(Box::new(MoveCursorCommand {
                direction: Direction::Right,
                extend_selection,
            })),

            "home" => Some(Box::new(MoveToLineStartCommand { extend_selection })),
            "end" => Some(Box::new(MoveToLineEndCommand { extend_selection })),

            "backspace" => Some(Box::new(DeleteBackwardCommand)),
            "delete" => Some(Box::new(DeleteForwardCommand)),
            "enter" => Some(Box::new(InsertNewlineCommand)),
            "tab" => Some(Box::new(InsertTabCommand)),
            "space" => Some(Box::new(InsertCharCommand { character: ' ' })),

            _ => {
                if key.len() == 1 && !modifiers.control && !modifiers.alt && !modifiers.platform {
                    if let Some(c) = key.chars().next() {
                        println!("CHAR INPUT: '{}' (code: {})", c, c as u32);
                        return Some(Box::new(InsertCharCommand { character: c }));
                    }
                }

                println!("Key not handled: '{}'", key);
                None
            }
        }
    }
}

