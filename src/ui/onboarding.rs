use gpui::{Context, IntoElement, div, img, px, rgb, prelude::*};
use gpui_component::{
    IconName,
    button::{Button, ButtonVariants},
    input::Input,
};

use crate::{
    assets::app_icon_path,
    palette::{
        ACCENT, APP_BG, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED, PANEL_MUTED, TEXT,
        TEXT_FAINT, TEXT_MUTED,
    },
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, _cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .size_full()
        .bg(rgb(APP_BG))
        .flex()
        .child(render_welcome_rail(_cx))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .child(render_search_bar(app, _cx))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_size(px(16.))
                                        .text_color(rgb(TEXT))
                                        .child("无连接"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child(
                                            "右键单击或单击“创建新连接...”按钮以创建新连接",
                                        ),
                                ),
                        ),
                ),
        )
}

fn render_welcome_rail(cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .w(px(280.))
        .h_full()
        .px_5()
        .py_7()
        .bg(rgb(PANEL_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .items_center()
        .child(
            div()
                .mt_3()
                .size(px(112.))
                .overflow_hidden()
                .child(img(app_icon_path()).size(px(112.))),
        )
        .child(
            div()
                .mt_3()
                .text_size(px(54.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(rgb(TEXT))
                .child("SuperTable"),
        )
        .child(
            div()
                .mt_1()
                .text_size(px(16.))
                .text_color(rgb(TEXT_MUTED))
                .child(format!("版本 {}", env!("CARGO_PKG_VERSION"))),
        )
        .child(
            div()
                .mt_5()
                .text_size(px(15.))
                .text_color(rgb(TEXT_FAINT))
                .child("现代数据库工作台"),
        )
        .child(
            div()
                .mt_8()
                .w_full()
                .flex()
                .flex_col()
                .gap_3()
                .child(action_row("备份数据库..."))
                .child(action_row("还原数据库..."))
                .child(action_row("创建连接...").on_click(
                    cx.listener(|app, _, window, cx| app.open_connection_form(window, cx)),
                )),
        )
}

fn render_search_bar(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .h(px(64.))
        .px_4()
        .flex()
        .items_center()
        .gap_3()
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .child(
            Button::new("create-connection")
                .ghost()
                .icon(IconName::Plus)
                .on_click(cx.listener(|app, _, window, cx| app.open_connection_form(window, cx))),
        )
        .child(
            div()
                .flex_1()
                .child(Input::new(&app.onboarding_search).prefix(IconName::Search)),
        )
}

fn action_row(label: &'static str) -> Button {
    Button::new(label)
        .ghost()
        .w_full()
        .child(
            div()
        .w_full()
        .h(px(56.))
        .px_4()
        .rounded(px(10.))
        .border_1()
        .border_color(rgb(BORDER))
        .bg(rgb(PANEL_ELEVATED))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .size(px(28.))
                        .rounded(px(8.))
                        .bg(rgb(PANEL_MUTED))
                        .border_1()
                        .border_color(rgb(ACCENT))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(TEXT))
                        .child("+"),
                )
                .child(div().text_size(px(15.)).text_color(rgb(TEXT)).child(label)),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(TEXT_FAINT))
                .child(""),
        ))
}
