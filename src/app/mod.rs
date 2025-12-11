use gpui::*;
use gpui_component::*;

use crate::editor::editor_component::EditorComponent;
use crate::ui::{status_bar::StatusBar, tab_bar::TabBar};

pub mod app_state;
pub mod commands;
pub mod shortcuts;

pub struct App;

impl App {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(cx.new(|cx| TabBar::new(cx)))
            .child(
                h_flex()
                    .flex_grow()
                    .child(cx.new(|cx| EditorComponent::new(cx))),
            )
            .child(cx.new(|cx| StatusBar::new(cx)))
    }
}
