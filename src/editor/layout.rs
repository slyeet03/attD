use gpui::*;
use gpui_component::*;

pub fn vertical_layout(children: Vec<impl IntoElement>) -> impl IntoElement {
    let mut container = v_flex().size_full();

    for child in children {
        container = container.child(child);
    }

    container
}

pub fn horizontal_layout(children: Vec<impl IntoElement>) -> impl IntoElement {
    let mut container = h_flex().size_full();

    for child in children {
        container = container.child(child);
    }

    container
}
