use crate::editor::editor_component::{self, EditorComponent};
use crate::ui::tab_bar::{self, TabInfo};
use crate::ui::{status_bar::StatusBar, tab_bar::TabBar};
use app_keyboard_handler::AppKeyBoardHandler;
use app_state::AppState;
use gpui::*;
use gpui_component::*;

pub mod app_keyboard_handler;
pub mod app_state;
pub mod commands;
pub mod shortcuts;

pub struct App {
    editor: Entity<EditorComponent>,
    tab_bar: Entity<TabBar>,
    status_bar: Entity<StatusBar>,
    app_state: AppState,
}

impl App {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| EditorComponent::new(cx));
        let tab_bar = cx.new(|cx| TabBar::new(cx));
        let status_bar = cx.new(|cx| StatusBar::new(cx));
        let mut app_state = AppState::default();

        cx.update_entity(&editor, |editor_component, _cx| {
            app_state.sync_current_tab_to_editor(editor_component);
        });

        Self {
            editor,
            tab_bar,
            status_bar,
            app_state,
        }
    }
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tab_info: Vec<TabInfo> = self
            .app_state
            .tabs
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| TabInfo {
                name: tab.display_name().to_string(),
                is_active: i == self.app_state.tabs.active,
                is_dirty: tab.dirty,
            })
            .collect();

        cx.update_entity(&self.tab_bar, |tab_bar, _| {
            tab_bar.update_tabs(tab_info);
        });

        v_flex()
            .size_full()
            .on_key_down(cx.listener(|app, event: &KeyDownEvent, window, cx| {
                let handled = AppKeyBoardHandler::handle_key_event(
                    &mut app.app_state,
                    &mut app.editor,
                    event,
                    cx,
                );

                if handled {
                    cx.notify();
                }
            }))
            .child(div().h(relative(0.05)).w_full().child(self.tab_bar.clone()))
            .child(
                div()
                    .h(relative(0.90))
                    .w_full()
                    .child(h_flex().size_full().child(self.editor.clone())),
            )
            .child(
                div()
                    .h(relative(0.05))
                    .w_full()
                    .child(self.status_bar.clone()),
            )
    }
}
