use super::app_state::{self, AppState};
use crate::editor::{self, EditorComponent, editor_component};
use gpui::{AppContext, Context, Entity, KeyDownEvent};
use rfd::FileDialog;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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
                    let path = FileDialog::new()
                        .set_title("Select a file to open")
                        .pick_file();

                    if let Some(file_path) = path {
                        cx.update_entity(editor, |editor_comp, _| {
                            app_state.open_file(file_path, editor_comp);
                        });
                    }

                    return true;
                }
                "s" => {
                    cx.update_entity(editor, |editor_comp, _| {
                        app_state.sync_editor_to_current_tab(editor_comp);
                        editor_comp.dirty_flag = false;
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
