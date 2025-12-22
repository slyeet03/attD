use gpui::{AppContext, Context, Entity, KeyDownEvent};

use crate::editor::{EditorComponent, editor_component};

use super::app_state::{self, AppState};

pub struct AppKeyBoardHandler;

impl AppKeyBoardHandler {
    pub fn handle_key_event(
        app_state: &mut AppState,
        editor: &Entity<EditorComponent>,
        event: &KeyDownEvent,
        cx: &mut Context<impl gpui::Render>,
    ) -> bool {
        let key = event.keystroke.key.as_str();
        let modifiers = &event.keystroke.modifiers;

        if modifiers.platform {
            match key {
                "t" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.create_new_tab(editor_comp);
                    });
                    return true;
                }
                "w" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.tabs.close_tab(app_state.tabs.active);
                    });
                    return true;
                }
                "tab" if modifiers.shift => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.tabs.prev_tab();
                    });
                    return true;
                }
                "tab" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.tabs.next_tab();
                    });
                    return true;
                }
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    let index = key.parse::<usize>().unwrap() - 1;
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.switch_to_tab(index, editor_comp);
                    });
                    return true;
                }
                _ => {}
            }
        }
        false
    }
}
