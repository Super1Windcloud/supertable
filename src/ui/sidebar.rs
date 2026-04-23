use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants},
};

use crate::{
    data::SCHEMA_ITEMS,
    palette::{
        ACCENT, ACCENT_SOFT, BLUE, BORDER, BORDER_SOFT, DANGER, PANEL_BG, PANEL_MUTED, TEXT,
        TEXT_FAINT, TEXT_MUTED, WARNING,
    },
};

use super::app::SuperTableApp;

pub fn render(app: &SuperTableApp, _cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .w(px(286.))
        .h_full()
        .bg(rgb(PANEL_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .child(
            div()
                .px_3()
                .py_3()
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(div().text_color(rgb(TEXT)).child("Connections"))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(format!("{} active endpoints", app.connections.len())),
                        ),
                )
                .child(Button::new("add-conn").ghost().label("+ Add").on_click(
                    _cx.listener(|app, _, window, cx| app.open_connection_form(window, cx)),
                )),
        )
        .child(
            div()
                .px_2()
                .py_2()
                .gap_2()
                .flex()
                .flex_col()
                .children(app.connections.iter().map(|item| {
                    let bg = if item.active { rgb(PANEL_MUTED) } else { rgb(PANEL_BG) };
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
                        .py_2()
                        .rounded(px(10.))
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
                                .gap_0p5()
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
                                        .child(item.meta()),
                                ),
                        )
                        .child(
                            Badge::new().count(1).color(badge_color).child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(999.))
                                    .bg(rgb(0x0F1318))
                                    .text_size(px(11.))
                                    .text_color(rgb(TEXT))
                                    .child(badge),
                            ),
                        )
                })),
        )
        .child(
            div()
                .mx_3()
                .mt_2()
                .mb_3()
                .border_t_1()
                .border_color(rgb(BORDER_SOFT)),
        )
        .child(
            div()
                .px_3()
                .pb_2()
                .flex()
                .flex_col()
                .child(div().text_color(rgb(TEXT)).child("Database Explorer"))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child("warehouse.production"),
                ),
        )
        .child(
            div()
                .px_2()
                .gap_1()
                .flex()
                .flex_col()
                .children(SCHEMA_ITEMS.into_iter().map(|item| {
                    let bg = if item.active { rgb(ACCENT_SOFT) } else { rgb(PANEL_BG) };
                    let fg = if item.active { rgb(ACCENT) } else { rgb(TEXT_MUTED) };
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(10.))
                        .bg(bg)
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_color(fg).child(item.name))
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(format!("{}", item.count)),
                        )
                })),
        )
}
