use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
};

use crate::{
    data::QUERY_LINES,
    palette::{
        ACCENT, ACCENT_SOFT, BLUE, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED, SURFACE_SOFT,
        TEXT, TEXT_FAINT, TEXT_MUTED,
    },
};

use super::app::SuperTableApp;

pub fn render_tabs(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let entity = cx.entity().clone();
    div()
        .rounded(px(18.))
        .bg(rgb(PANEL_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .child(
            TabBar::new("editor-tabs")
                .underline()
                .selected_index(app.selected_editor_tab)
                .on_click(move |ix, _, cx| {
                    entity.update(cx, |this, cx| {
                        this.selected_editor_tab = *ix;
                        cx.notify();
                    });
                })
                .child(Tab::new().label("orders.sql"))
                .child(Tab::new().label("customers.sql"))
                .child(Tab::new().label("insights.sql")),
        )
}

pub fn render_sql_editor() -> impl IntoElement {
    div()
        .flex_1()
        .rounded(px(18.))
        .bg(rgb(PANEL_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .child(
            div()
                .h(px(52.))
                .px_4()
                .flex()
                .items_center()
                .justify_between()
                .bg(rgb(PANEL_ELEVATED))
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(Button::new("run").primary().label("Run"))
                        .child(Button::new("format").ghost().label("Format"))
                        .child(Button::new("explain").ghost().label("Explain")),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(999.))
                                .bg(rgb(ACCENT_SOFT))
                                .text_size(px(11.))
                                .text_color(rgb(ACCENT))
                                .child("Connected"),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child("Ctrl+Enter to execute selection"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .h_full()
                .bg(rgb(SURFACE_SOFT))
                .child(
                    div()
                        .w(px(52.))
                        .h_full()
                        .bg(rgb(PANEL_BG))
                        .border_r_1()
                        .border_color(rgb(BORDER_SOFT))
                        .pt_4()
                        .children((1..=QUERY_LINES.len()).map(|line| {
                            div()
                                .h(px(24.))
                                .pr_3()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .text_right()
                                .child(format!("{line}"))
                        })),
                )
                .child(
                    div()
                        .flex_1()
                        .pt_4()
                        .px_4()
                        .child(
                            div()
                                .mb_3()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(rgb(TEXT_MUTED))
                                        .child("Live draft"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child("warehouse.orders"),
                                ),
                        )
                        .children(QUERY_LINES.into_iter().enumerate().map(|(ix, line)| {
                            let color = if ix == 0 || ix == 6 {
                                rgb(BLUE)
                            } else if line.contains("paid") || line.contains("refunded") {
                                rgb(ACCENT)
                            } else {
                                rgb(TEXT)
                            };

                            div()
                                .h(px(24.))
                                .text_size(px(13.))
                                .text_color(color)
                                .child(line)
                        })),
                ),
        )
}
