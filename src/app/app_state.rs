use crate::editor::{EditorComponent, EditorState, editor_component};
use crate::tabs::{Tab, TabManager};

pub struct AppState {
    pub tabs: TabManager,
    pub editor: EditorState,
}

impl Default for AppState {
    fn default() -> Self {
        let mut tab_manager = TabManager {
            tabs: Vec::new(),
            active: 0,
        };

        let first_tab = Tab::new_empty(0);
        tab_manager.add_tab(first_tab);

        Self {
            tabs: tab_manager,
            editor: EditorState::default(),
        }
    }
}

impl AppState {
    pub fn sync_editor_to_current_tab(&mut self, editor_component: &EditorComponent) {
        if let Some(active_tab) = self.tabs.active_tab_mut() {
            active_tab.save_editor_state(
                editor_component.buffer.text.clone(),
                editor_component.cursor.row,
                editor_component.cursor.col,
                editor_component.selection.anchor.row,
                editor_component.selection.anchor.col,
                editor_component.scroll_offset.width,
                editor_component.scroll_offset.height,
                true,
            );
        }
    }

    pub fn sync_current_tab_to_editor(&self, editor_component: &mut EditorComponent) {
        if let Some(active_tab) = self.tabs.active_tab() {
            let editor_state = active_tab.get_editor_state();
            editor_component.buffer.text = editor_state.0;
            editor_component.cursor.row = editor_state.1;
            editor_component.cursor.col = editor_state.2;
            editor_component.selection.anchor.row = editor_state.3;
            editor_component.selection.anchor.col = editor_state.4;
            editor_component.scroll_offset.width = editor_state.5;
            editor_component.scroll_offset.height = editor_state.6;
            // no previous tab shit gets to the current tab
            editor_component.buffer.undo_stack.clear();
            editor_component.buffer.redo_stack.clear();
        }
    }

    pub fn switch_to_tab_and_sync(&mut self, index: usize, editor_component: &mut EditorComponent) {
        self.sync_editor_to_current_tab(editor_component);
        self.tabs.activate(index);
        self.sync_current_tab_to_editor(editor_component);
    }

    pub fn create_new_tab(&mut self, editor_component: &mut EditorComponent) {
        self.sync_editor_to_current_tab(editor_component);
        let next_id = self.tabs.tabs.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let new_tab = Tab::new_empty(next_id);
        self.tabs.add_tab(new_tab);
        self.sync_current_tab_to_editor(editor_component);
    }

    pub fn switch_to_tab(&mut self, index: usize, editor_component: &mut EditorComponent) {
        if index < self.tabs.tabs.len() && index != self.tabs.active {
            self.sync_editor_to_current_tab(editor_component);
            self.tabs.activate(index);
            self.sync_current_tab_to_editor(editor_component);
        }
    }
}
