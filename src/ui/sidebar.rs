use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::button::{Button, ButtonVariants};

use crate::{
    data::ConnectionKind,
    palette::{
        ACCENT, ACCENT_SOFT, BLUE, BORDER, BORDER_SOFT, DANGER, PANEL_MUTED, SIDEBAR_BG,
        SURFACE_SOFT, TEXT, TEXT_FAINT, TEXT_MUTED, WARNING,
    },
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .w(px(332.))
        .h_full()
        .bg(rgb(SIDEBAR_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .child(render_header(app, cx))
        .child(render_sources(app, cx))
        .child(render_objects(app))
}

fn render_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .px_4()
        .py_4()
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().text_color(rgb(TEXT)).child("SuperTable"))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(locale.sources()),
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
                        .child("sqlx"),
                ),
        )
        .child(
            div()
                .rounded(px(14.))
                .bg(rgb(SURFACE_SOFT))
                .border_1()
                .border_color(rgb(BORDER_SOFT))
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .border_b_1()
                        .border_color(rgb(BORDER_SOFT))
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_size(px(11.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(div().child(locale.sources()))
                        .child(div().child(locale.objects())),
                )
                .child(
                    div()
                        .px_3()
                        .py_3()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_color(rgb(TEXT))
                                .child(locale.configured_endpoints(app.connections.len())),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(format!("{}", app.preview.schema_items.len())),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    Button::new("add-conn")
                        .ghost()
                        .label(locale.create_connection())
                        .on_click(cx.listener(|app, _, window, cx| {
                            app.open_connection_form(window, cx)
                        })),
                )
                .child(
                    Button::new("refresh-preview")
                        .ghost()
                        .label(locale.refresh())
                        .on_click(cx.listener(|app, _, _, cx| {
                            app.refresh_preview(cx);
                        })),
                ),
        )
}

fn render_sources(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .px_3()
        .py_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .px_2()
                .pb_2()
                .flex()
                .items_center()
                .justify_between()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(div().child("NAME"))
                .child(div().child("TYPE")),
        )
        .children(app.connections.iter().enumerate().map(|(index, item)| {
            let is_active = item.active;
            let bg = if is_active {
                rgb(PANEL_MUTED)
            } else {
                rgb(SIDEBAR_BG)
            };
            let accent_bar = if is_active {
                rgb(ACCENT)
            } else {
                rgb(BORDER_SOFT)
            };
            let badge_color = match item.kind {
                ConnectionKind::MySql => rgb(DANGER),
                ConnectionKind::PostgreSql | ConnectionKind::MongoDb => rgb(BLUE),
                _ => rgb(WARNING),
            };

            div()
                .rounded(px(12.))
                .border_1()
                .border_color(if is_active {
                    rgb(ACCENT)
                } else {
                    rgb(BORDER_SOFT)
                })
                .bg(bg)
                .overflow_hidden()
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |app, _, _, cx| {
                    app.activate_connection(index, cx);
                }))
                .child(
                    div()
                        .h(px(64.))
                        .flex()
                        .child(div().w(px(4.)).h_full().bg(accent_bar))
                        .child(
                            div()
                                .flex_1()
                                .px_3()
                                .py_3()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(div().text_color(rgb(TEXT)).child(item.name.clone()))
                                        .child(
                                            div()
                                                .text_size(px(12.))
                                                .text_color(rgb(TEXT_MUTED))
                                                .child(item.meta(app.locale)),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.))
                                                .text_color(rgb(TEXT_FAINT))
                                                .child(item.endpoint()),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .rounded(px(999.))
                                        .bg(badge_color)
                                        .text_size(px(10.))
                                        .text_color(rgb(SIDEBAR_BG))
                                        .child(item.kind.badge()),
                                ),
                        ),
                )
        }))
}

fn render_objects(app: &SuperTableApp) -> impl IntoElement {
    div()
        .flex_1()
        .px_3()
        .pt_2()
        .pb_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .px_2()
                .pt_2()
                .pb_2()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(app.locale.database_explorer()),
        )
        .children(app.preview.schema_items.iter().map(|item| {
            let active = item.active;
            div()
                .h(px(42.))
                .rounded(px(10.))
                .bg(if active {
                    rgb(ACCENT_SOFT)
                } else {
                    rgb(SURFACE_SOFT)
                })
                .border_1()
                .border_color(if active {
                    rgb(ACCENT)
                } else {
                    rgb(BORDER_SOFT)
                })
                .px_3()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_color(if active { rgb(ACCENT) } else { rgb(TEXT_MUTED) })
                        .child(app.locale.schema_item(&item.name)),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(format!("{}", item.count)),
                )
        }))
}
