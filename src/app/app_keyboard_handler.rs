use std::path::PathBuf;

use gpui::{AppContext, Context, Entity, KeyDownEvent};

use crate::editor::{self, EditorComponent, editor_component};

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
                        app_state.close_current_tab(editor_comp);
                    });
                    return true;
                }
                "o" => {
                    // simulating a file opening right now
                    // TODO: open file dialog
                    println!("inside the o match case");
                    let path = PathBuf::from("test.txt");
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.open_file(path, editor_comp);
                    });
                    return true;
                }
                "s" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.sync_editor_to_current_tab(editor_comp);
                    });

                    match app_state.save_current_file() {
                        Ok(_) => {
                            println!("File saved successfully!");
                            cx.notify();
                        }
                        Err(e) => println!("Failed to save: {}", e),
                    }
                    return true;
                }
                "tab" if modifiers.shift => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.previous_tab(editor_comp);
                    });
                    return true;
                }
                "tab" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.next_tab(editor_comp);
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
