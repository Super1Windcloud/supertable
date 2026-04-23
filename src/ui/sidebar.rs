use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants},
};

use crate::{
    data::SCHEMA_ITEMS,
    palette::{
        ACCENT, ACCENT_SOFT, BLUE, BORDER, BORDER_SOFT, DANGER, PANEL_ELEVATED, PANEL_MUTED,
        SIDEBAR_BG, SURFACE_SOFT, TEXT, TEXT_FAINT, TEXT_MUTED, WARNING,
    },
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, _cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .w(px(320.))
        .h_full()
        .bg(rgb(SIDEBAR_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .child(
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
                        .flex_col()
                        .gap_1()
                        .child(div().text_color(rgb(TEXT)).child(locale.connections()))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(locale.configured_endpoints(app.connections.len())),
                        ),
                )
                .child(
                    div()
                        .rounded(px(14.))
                        .bg(rgb(PANEL_ELEVATED))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_MUTED))
                                .child(locale.today()),
                        )
                        .child(div().text_color(rgb(TEXT)).child(locale.sync_healthy()))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(locale.latency_hint()),
                        )
                        .child(
                            Button::new("add-conn")
                                .ghost()
                                .label(locale.create_connection())
                                .on_click(_cx.listener(|app, _, window, cx| {
                                    app.open_connection_form(window, cx)
                                })),
                        ),
                ),
        )
        .child(
            div()
                .px_3()
                .py_3()
                .gap_3()
                .flex()
                .flex_col()
                .children(app.connections.iter().map(|item| {
                    let bg = if item.active { rgb(PANEL_MUTED) } else { rgb(SURFACE_SOFT) };
                    let border = if item.active { rgb(ACCENT) } else { rgb(BORDER_SOFT) };
                    let badge = item.kind.badge();
                    let badge_color = if badge == "MYSQL" {
                        rgb(DANGER)
                    } else if badge == "PG" || badge == "MONGO" {
                        rgb(BLUE)
                    } else {
                        rgb(WARNING)
                    };

                    div()
                        .px_3()
                        .py_3()
                        .rounded(px(14.))
                        .border_1()
                        .border_color(border)
                        .bg(bg)
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
                                        .child(item.endpoint()),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child(item.meta(locale)),
                                ),
                        )
                        .child(
                            Badge::new().count(1).color(badge_color).child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(999.))
                                    .bg(rgb(SIDEBAR_BG))
                                    .text_size(px(11.))
                                    .text_color(rgb(TEXT))
                                    .child(badge),
                            ),
                        )
                })),
        )
        .child(
            div()
                .mx_4()
                .mt_1()
                .mb_4()
                .border_t_1()
                .border_color(rgb(BORDER_SOFT)),
        )
        .child(
            div()
                .px_4()
                .pb_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_color(rgb(TEXT)).child(locale.database_explorer()))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child("warehouse.production / analytics"),
                ),
        )
        .child(
            div()
                .px_3()
                .gap_2()
                .flex()
                .flex_col()
                .children(SCHEMA_ITEMS.into_iter().map(|item| {
                    let bg = if item.active { rgb(ACCENT_SOFT) } else { rgb(SIDEBAR_BG) };
                    let fg = if item.active { rgb(ACCENT) } else { rgb(TEXT_MUTED) };
                    div()
                        .px_3()
                        .py_3()
                        .rounded(px(12.))
                        .bg(bg)
                        .border_1()
                        .border_color(if item.active { rgb(ACCENT) } else { rgb(BORDER_SOFT) })
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_color(fg).child(locale.schema_item(item.name)))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(format!("{}", item.count)),
                        )
                })),
        )
}
