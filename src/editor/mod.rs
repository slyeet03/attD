pub mod buffer;
pub mod cursor;
pub mod editor_component;
pub mod input;
pub mod log;
pub mod selection;
pub use buffer::Buffer;
pub use cursor::Cursor;
pub use editor_component::EditorComponent;
pub use selection::Selection;

pub struct EditorSettings {
    pub font_size: usize,
    pub theme: Theme,
    pub wrap: bool,
    pub indent_size: usize,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: 16,
            theme: Theme,
            wrap: false,
            indent_size: 4,
        }
    }
}

pub struct EditorStatus {
    pub modified_flag: bool,
    pub search_mode: bool,
}

impl Default for EditorStatus {
    fn default() -> Self {
        Self {
            modified_flag: false,
            search_mode: false,
        }
    }
}

pub struct EditorState {
    pub settings: EditorSettings,
    pub status: EditorStatus,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            settings: EditorSettings::default(),
            status: EditorStatus::default(),
        }
    }
}

pub struct Theme;
