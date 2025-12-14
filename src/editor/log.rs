use super::EditorComponent;
use gpui::KeyDownEvent;

pub fn log_keystrokes(editor: &EditorComponent, event: &KeyDownEvent) {
    println!("KEY DOWN EVENT FIRED!");
    println!(
        "KEY PRESSED: '{}' | Modifiers: ctrl={} alt={} shift={} platform={}",
        event.keystroke.key,
        event.keystroke.modifiers.control,
        event.keystroke.modifiers.alt,
        event.keystroke.modifiers.shift,
        event.keystroke.modifiers.platform
    );
    println!(
        "BEFORE - Cursor: row={}, col={} | Buffer lines: {}",
        editor.cursor.row,
        editor.cursor.col,
        editor.buffer.line_count()
    );
}
