use super::Tab;
use std::path::PathBuf;

pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active: usize,
}

impl TabManager {
    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
        self.active = self.tabs.len() - 1;
    }

    pub fn open_file(&mut self, path: PathBuf) {
        let tab = Tab::new_from_file(self.tabs.len(), path);
        self.add_tab(tab);
    }

    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active >= self.tabs.len() && !self.tabs.is_empty() {
                self.active = self.tabs.len() - 1;
            }
        }
    }

    pub fn activate(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active = index;
        }
    }

    pub fn save_current_file(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            if let Some(path) = &tab.path {
                //tab.buffer.save_to_file(path);
                tab.dirty = false;
            }
        }
    }

    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + 1) % self.tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + self.tabs.len() - 1) % self.tabs.len();
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active)
    }
}
