use crate::editor::EditorState;
use crate::tabs::{Tab, TabManager};
use std::path::PathBuf;

pub struct AppState {
    pub tabs: TabManager,
    pub editor: EditorState,
}

impl AppState {
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.active_tab()
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.active_tab_mut()
    }

    pub fn switch_tab(&mut self, index: usize) {
        self.tabs.activate(index)
    }

    pub fn close_tab(&mut self, index: usize) {
        self.tabs.close_tab(index)
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) {
        self.tabs.open_file(path.unwrap())
    }

    pub fn save_current_file(&mut self) {
        self.tabs.save_current_file()
    }
}
