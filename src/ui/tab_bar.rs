use gpui::*;
use gpui_component::*;

pub struct TabBar;

impl TabBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for TabBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child("tab bar")
    }
}
