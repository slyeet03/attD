use crate::editor::editor_component::EditorComponent;
use crate::ui::{status_bar::StatusBar, tab_bar::TabBar};
use gpui::*;
use gpui_component::*;

pub mod app_state;
pub mod commands;
pub mod shortcuts;

pub struct App {
    editor: Entity<EditorComponent>,
    tab_bar: Entity<TabBar>,
    status_bar: Entity<StatusBar>,
}

impl App {
    pub fn new(cx: &mut Context<Self>) -> Self {
        println!("App::new() called!");
        Self {
            editor: cx.new(|cx| EditorComponent::new(cx)),
            tab_bar: cx.new(|cx| TabBar::new(cx)),
            status_bar: cx.new(|cx| StatusBar::new(cx)),
        }
    }
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("App::render() called!");

        v_flex()
            .size_full()
            .child(div().h(relative(0.10)).w_full().child(self.tab_bar.clone()))
            .child(
                div()
                    .h(relative(0.85))
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
