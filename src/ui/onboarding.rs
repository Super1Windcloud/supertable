use gpui::{Context, IntoElement, div, img, px, rgb, prelude::*};
use gpui_component::{
    IconName,
    button::{Button, ButtonVariants},
    input::Input,
};

use crate::{
    assets::app_icon_path,
    palette::{
        ACCENT, ACCENT_SOFT, APP_BG, APP_BG_ALT, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED,
        PANEL_MUTED, SURFACE_SOFT, TEXT, TEXT_FAINT, TEXT_MUTED,
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
                                .w(px(620.))
                                .rounded(px(28.))
                                .border_1()
                                .border_color(rgb(BORDER))
                                .bg(rgb(SURFACE_SOFT))
                                .p_7()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .rounded(px(999.))
                                        .bg(rgb(ACCENT_SOFT))
                                        .text_size(px(12.))
                                        .text_color(rgb(ACCENT))
                                        .child("Get Started"),
                                )
                                .child(
                                    div()
                                        .text_size(px(30.))
                                        .text_color(rgb(TEXT))
                                        .child("建立你的第一个数据工作区"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child("添加连接后，你可以立即浏览结构、编写 SQL，并在统一结果面板中查看输出。"),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .flex()
                                        .gap_3()
                                        .child(action_row("创建连接...").on_click(
                                            _cx.listener(|app, _, window, cx| {
                                                app.open_connection_form(window, cx)
                                            }),
                                        ))
                                        .child(action_row("导入示例数据")),
                                ),
                        ),
                ),
        )
}

fn render_welcome_rail(cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .w(px(360.))
        .h_full()
        .px_6()
        .py_8()
        .bg(rgb(PANEL_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .child(
            div()
                .size(px(84.))
                .rounded(px(24.))
                .bg(rgb(APP_BG_ALT))
                .border_1()
                .border_color(rgb(BORDER))
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .child(img(app_icon_path()).size(px(68.))),
        )
        .child(
            div()
                .mt_5()
                .text_size(px(44.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(rgb(TEXT))
                .child("SuperTable"),
        )
        .child(
            div()
                .mt_1()
                .text_size(px(14.))
                .text_color(rgb(TEXT_MUTED))
                .child(format!("版本 {}", env!("CARGO_PKG_VERSION"))),
        )
        .child(
            div()
                .mt_6()
                .text_size(px(15.))
                .text_color(rgb(TEXT_FAINT))
                .child("为多种数据库连接、探索与查询打造的现代工作台"),
        )
        .child(
            div()
                .mt_8()
                .w_full()
                .flex()
                .flex_col()
                .gap_3()
                .child(info_card("Fast setup", "MySQL / PostgreSQL / Redis / MongoDB / SQLite"))
                .child(info_card("Focused workflow", "Connections, editor and results in one calm layout"))
                .child(action_row("创建连接...").on_click(cx.listener(|app, _, window, cx| {
                    app.open_connection_form(window, cx)
                }))),
        )
}

fn render_search_bar(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .h(px(72.))
        .px_5()
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
    Button::new(label).ghost().child(
        div()
            .w(px(220.))
            .h(px(56.))
            .px_4()
            .rounded(px(14.))
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
            ),
    )
}

fn info_card(title: &'static str, body: &'static str) -> impl IntoElement {
    div()
        .w_full()
        .rounded(px(16.))
        .border_1()
        .border_color(rgb(BORDER))
        .bg(rgb(PANEL_ELEVATED))
        .p_4()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_color(rgb(TEXT)).child(title))
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(TEXT_FAINT))
                .child(body),
        )
}
