mod assets;
mod data;
mod palette;
mod ui;

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::{Root, theme::{Theme, ThemeMode}};

use ui::app::SuperTableApp;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);

        let bounds = Bounds::centered(None, size(px(1440.), px(920.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some("com.supertable.desktop".to_string()),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| SuperTableApp::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
