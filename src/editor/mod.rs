pub mod buffer;
pub use buffer::Buffer;
pub mod cursor;
pub mod editor_component;
pub mod layout;
pub mod selection;
pub use editor_component::EditorComponent;

pub struct EditorSettings {
    pub font_size: usize,
    pub theme: Theme,
    pub wrap: bool,
    pub indent_size: usize,
}

pub struct EditorStatus {
    pub modified_flag: bool,
    pub search_mode: bool,
}

pub struct EditorState {
    pub settings: EditorSettings,
    pub status: EditorStatus,
}

enum Key {
    Left,
    Right,
    Up,
    Down,
    Char(char),
    Backspace,
    Delete,
    Enter,
}

pub struct Theme;
