use gpui::{Context, IntoElement, div, img, px, rgb, prelude::*};
use gpui_component::{
    button::{Button, ButtonVariants},
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
    let locale = app.locale;

    div()
        .size_full()
        .bg(rgb(APP_BG))
        .flex()
        .child(render_welcome_rail(app, _cx))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
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
                                        .child(locale.get_started()),
                                )
                                .child(
                                    div()
                                        .text_size(px(30.))
                                        .text_color(rgb(TEXT))
                                        .child(locale.first_workspace()),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child(locale.onboarding_intro()),
                                )
                                .child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .rounded(px(999.))
                                        .bg(rgb(PANEL_ELEVATED))
                                        .border_1()
                                        .border_color(rgb(BORDER))
                                        .text_size(px(12.))
                                        .text_color(rgb(TEXT_MUTED))
                                        .child(locale.open_source_badge()),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .flex()
                                        .gap_3()
                                        .child(action_row(locale.create_connection_title()).on_click(
                                            _cx.listener(|app, _, window, cx| {
                                                app.open_connection_form(window, cx)
                                            }),
                                        ))
                                        .child(action_row(locale.import_sample_data())),
                                ),
                        ),
                ),
        )
}

fn render_welcome_rail(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

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
                .flex()
                .justify_end()
                .child(
                    Button::new("toggle-locale")
                        .ghost()
                        .label(locale.switch_label())
                        .on_click(cx.listener(|app, _, window, cx| app.toggle_locale(window, cx))),
                ),
        )
        .child(
            div()
                .mt_4()
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
                .child(format!("{} {}", locale.version(), env!("CARGO_PKG_VERSION"))),
        )
        .child(
            div()
                .mt_6()
                .text_size(px(15.))
                .text_color(rgb(TEXT_FAINT))
                .child(locale.welcome_copy()),
        )
        .child(
            div()
                .mt_8()
                .w_full()
                .flex()
                .flex_col()
                .gap_3()
                .child(info_card(locale.info_fast_setup_title(), locale.info_fast_setup_body()))
                .child(info_card(
                    locale.info_focused_workflow_title(),
                    locale.info_focused_workflow_body(),
                ))
                .child(action_row(locale.create_connection_title()).on_click(
                    cx.listener(|app, _, window, cx| {
                        app.open_connection_form(window, cx)
                    }),
                )),
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
