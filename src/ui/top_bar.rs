use gpui::{
    Context, IntoElement, img, px, rgb, div, prelude::FluentBuilder as _,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    input::Input,
};

use crate::{
    assets::app_icon_path,
    palette::{ACCENT, BORDER, PANEL_BG, TEXT, TEXT_FAINT},
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, _cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .h(px(56.))
        .px_4()
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
                .gap_3()
                .child(
                    div()
                        .size(px(32.))
                        .rounded(px(9.))
                        .bg(rgb(0x0C1512))
                        .border_1()
                        .border_color(rgb(ACCENT))
                        .overflow_hidden()
                        .child(img(app_icon_path()).size(px(32.))),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(div().text_color(rgb(TEXT)).child("SuperTable"))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child("TablePlus-inspired workspace"),
                        ),
                ),
        )
        .child(
            div()
                .w(px(420.))
                .child(Input::new(&app.global_search).cleanable(true)),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(Button::new("new-query").primary().label("New Query"))
                .child(Button::new("import").ghost().label("Import"))
                .child(Button::new("share").ghost().label("Share")),
        )
}
