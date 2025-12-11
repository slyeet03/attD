pub struct AppState {
    pub tabs: TabManager,
    pub editor: EditorState,
}

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
pub struct Theme {}
