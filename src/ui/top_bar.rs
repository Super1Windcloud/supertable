use gpui::{
    Context, IntoElement, div, img, px, rgb, prelude::*,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    input::Input,
};

use crate::{
    assets::app_icon_path,
    palette::{ACCENT, ACCENT_SOFT, BORDER, PANEL_BG, PANEL_ELEVATED, TEXT, TEXT_FAINT, TEXT_MUTED},
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, _cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .h(px(72.))
        .px_5()
        .py_3()
        .flex()
        .items_center()
        .justify_between()
        .bg(rgb(PANEL_BG))
        .border_b_1()
        .border_color(rgb(BORDER))
        .child(
            div()
                .flex()
                .items_center()
                .gap_4()
                .child(
                    div()
                        .size(px(40.))
                        .rounded(px(12.))
                        .bg(rgb(ACCENT_SOFT))
                        .border_1()
                        .border_color(rgb(ACCENT))
                        .overflow_hidden()
                        .child(img(app_icon_path()).size(px(40.))),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_size(px(18.))
                                .text_color(rgb(TEXT))
                                .child("SuperTable"),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(locale.app_tagline()),
                        ),
                ),
        )
        .child(
            div()
                .w(px(460.))
                .rounded(px(14.))
                .bg(rgb(PANEL_ELEVATED))
                .border_1()
                .border_color(rgb(BORDER))
                .px_1()
                .child(Input::new(&app.global_search).cleanable(true)),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    Button::new("toggle-locale")
                        .ghost()
                        .label(locale.switch_label())
                        .on_click(_cx.listener(|app, _, window, cx| {
                            app.toggle_locale(window, cx)
                        })),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(999.))
                        .bg(rgb(PANEL_ELEVATED))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_MUTED))
                        .child(locale.workspaces_label()),
                )
                .child(Button::new("new-query").primary().label(locale.new_query()))
                .child(
                    Button::new("import")
                        .ghost()
                        .label(locale.add_connection())
                        .on_click(_cx.listener(|app, _, window, cx| {
                            app.open_connection_form(window, cx)
                        })),
                )
                .child(Button::new("share").ghost().label(locale.export())),
        )
}
