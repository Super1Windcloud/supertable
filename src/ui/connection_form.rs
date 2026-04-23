use gpui::{Context, IntoElement, Window, div, px, rgb, prelude::*};
use gpui_component::{
    Selectable,
    button::{Button, ButtonVariants},
    input::Input,
};

use crate::{
    data::ConnectionKind,
    palette::{
        ACCENT, ACCENT_SOFT, APP_BG, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED, TEXT,
        TEXT_FAINT, TEXT_MUTED,
    },
};

use super::app::SuperTableApp;

pub fn render(
    app: &SuperTableApp,
    window: &mut Window,
    cx: &mut Context<SuperTableApp>,
) -> impl IntoElement {
    div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .bottom_0()
        .bg(rgb(APP_BG))
        .child(
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .p_5()
                .child(
                    div()
                        .w(px(760.))
                        .rounded(px(24.))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .bg(rgb(PANEL_BG))
                        .overflow_hidden()
                        .child(render_header(app, cx))
                        .child(render_kind_selector(app, window, cx))
                        .child(render_form_fields(app))
                        .child(render_footer(app, cx)),
                ),
        )
}

fn render_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .px_5()
        .py_5()
        .border_b_1()
        .border_color(rgb(BORDER))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(22.))
                        .text_color(rgb(TEXT))
                        .child(locale.create_connection_title()),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(locale.configure_connection(app.selected_connection_kind.label())),
                ),
        )
        .child(
            Button::new("close-connection-form")
                .ghost()
                .label(locale.close())
                .on_click(cx.listener(|app, _, _, cx| {
                    app.close_connection_form();
                    cx.notify();
                })),
        )
}

fn render_kind_selector(
    app: &SuperTableApp,
    _window: &mut Window,
    cx: &mut Context<SuperTableApp>,
) -> impl IntoElement {
    div()
        .px_5()
        .py_4()
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .gap_2()
        .flex_wrap()
        .children(ConnectionKind::ALL.into_iter().map(|kind| {
            Button::new(kind.label())
                .ghost()
                .label(kind.label())
                .selected(app.selected_connection_kind == kind)
                .on_click(cx.listener(move |app, _, window, cx| {
                    app.set_connection_kind(kind, window, cx);
                }))
        }))
}

fn render_form_fields(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;
    let is_sqlite = app.selected_connection_kind == ConnectionKind::Sqlite;

    div()
        .px_5()
        .py_5()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .rounded(px(16.))
                .border_1()
                .border_color(rgb(BORDER))
                .bg(rgb(PANEL_ELEVATED))
                .px_4()
                .py_3()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_size(px(13.))
                                .text_color(rgb(TEXT))
                                .child(locale.connection_preset()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_MUTED))
                                .child(locale.preset_hint()),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(999.))
                        .bg(rgb(ACCENT_SOFT))
                        .text_size(px(11.))
                        .text_color(rgb(ACCENT))
                        .child(app.selected_connection_kind.label()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(field_label(locale.name()))
                .child(Input::new(&app.connection_name).cleanable(true)),
        )
        .when(!is_sqlite, |this| {
            this.child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(field_label(locale.host()))
                            .child(Input::new(&app.connection_host).cleanable(true)),
                    )
                    .child(
                        div()
                            .w(px(140.))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(field_label(locale.port()))
                            .child(Input::new(&app.connection_port).cleanable(true)),
                    ),
            )
        })
        .when(is_sqlite, |this| {
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(field_label(locale.database_file()))
                    .child(Input::new(&app.connection_file_path).cleanable(true)),
            )
        })
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(field_label(if is_sqlite {
                    locale.tag_or_note()
                } else {
                    locale.database()
                }))
                .child(Input::new(&app.connection_database).cleanable(true)),
        )
        .when(!is_sqlite, |this| {
            this.child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(field_label(locale.username()))
                            .child(Input::new(&app.connection_username).cleanable(true)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(field_label(locale.password()))
                            .child(Input::new(&app.connection_password).cleanable(true)),
                    ),
            )
        })
        .child(
            div()
                .rounded(px(12.))
                .bg(rgb(PANEL_ELEVATED))
                .border_1()
                .border_color(rgb(BORDER))
                .px_4()
                .py_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(rgb(TEXT))
                        .child(locale.supported_connection_types()),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_MUTED))
                        .child("MySQL, SQLite, PostgreSQL, Redis, MongoDB"),
                ),
        )
}

fn render_footer(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .px_5()
        .py_4()
        .border_t_1()
        .border_color(rgb(BORDER))
        .flex()
        .justify_end()
        .gap_3()
        .child(
            Button::new("cancel-connection")
                .ghost()
                .label(locale.cancel())
                .on_click(cx.listener(|app, _, _, cx| {
                    app.close_connection_form();
                    cx.notify();
                })),
        )
        .child(
            Button::new("save-connection")
                .primary()
                .label(locale.save_connection())
                .on_click(cx.listener(|app, _, window, cx| app.save_connection(window, cx))),
        )
}

fn field_label(text: &'static str) -> impl IntoElement {
    div().text_size(px(13.)).text_color(rgb(TEXT_FAINT)).child(text)
}
