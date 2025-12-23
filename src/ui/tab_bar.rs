use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::*;

pub struct TabClickedEvent {
    pub tab_index: usize,
}

pub struct TabInfo {
    pub name: String,
    pub is_active: bool,
    pub is_dirty: bool,
}

pub struct TabBar {
    pub tabs: Vec<TabInfo>,
}

impl EventEmitter<TabClickedEvent> for TabBar {}

impl TabBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { tabs: Vec::new() }
    }

    pub fn update_tabs(&mut self, tabs: Vec<TabInfo>) {
        self.tabs = tabs;
    }

    fn render_tab(
        &self,
        index: usize,
        tab_info: &TabInfo,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .bg(if tab_info.is_active {
                rgb(0x1e1e1e)
            } else {
                rgb(0x2d2d30)
            })
            .when(!tab_info.is_active, |this| {
                this.hover(|style| style.bg(rgb(0x3e3e42)))
            })
            .text_color(if tab_info.is_active {
                rgb(0xffffff)
            } else {
                rgb(0xcccccc)
            })
            .border_r_1()
            .border_color(rgb(0x1e1e1e))
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |_tab_bar, _event: &MouseDownEvent, _window, cx| {
                    cx.emit(TabClickedEvent { tab_index: index });
                }),
            )
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        div()
                            .when(tab_info.is_active, |this| {
                                this.font_weight(gpui::FontWeight::BOLD)
                            })
                            .child(tab_info.name.clone()),
                    )
                    .when(tab_info.is_dirty, |this| {
                        this.child(div().text_color(rgb(0xff6b6b)).child("●"))
                    }),
            )
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = h_flex().w_full().h_full().bg(rgb(0x2d2d30));

        // Iterate and add children one by one
        for (index, tab_info) in self.tabs.iter().enumerate() {
            container = container.child(self.render_tab(index, tab_info, cx));
        }

        container
    }
}
