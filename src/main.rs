use anyhow::Result;
use gpui::*;
use gpui_component::{Root, *};

pub mod app;
pub mod editor;
pub mod tabs;
pub mod ui;
use app::App;

fn main() {
    let application = Application::new().with_assets(gpui_component_assets::Assets);

    application.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| App::new(cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
