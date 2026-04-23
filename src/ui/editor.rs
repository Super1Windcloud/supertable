use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
};

use crate::palette::{
    ACCENT, ACCENT_SOFT, BLUE, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED, SURFACE_SOFT,
    TABLE_BG, TEXT, TEXT_FAINT, TEXT_MUTED,
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
                .child(Tab::new().label("source.sql"))
                .child(Tab::new().label("inspect.sql"))
                .child(Tab::new().label("notes.sql")),
        )
}

pub fn render_sql_editor(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;
    let query_lines = if app.preview.query_lines.is_empty() {
        vec!["-- Connect to a database source to load query preview".to_string()]
    } else {
        app.preview.query_lines.clone()
    };

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
                        .child(Button::new("run").primary().label(locale.run()))
                        .child(Button::new("format").ghost().label(locale.format()))
                        .child(Button::new("explain").ghost().label(locale.explain())),
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
                                .child(locale.connected()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(TEXT_FAINT))
                                .child(locale.execute_hint()),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .h_full()
                .bg(rgb(TABLE_BG))
                .child(
                    div()
                        .w(px(52.))
                        .h_full()
                        .bg(rgb(SURFACE_SOFT))
                        .border_r_1()
                        .border_color(rgb(BORDER_SOFT))
                        .pt_4()
                        .children((1..=query_lines.len()).map(|line| {
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
                                        .child(locale.live_draft()),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(rgb(TEXT_FAINT))
                                        .child(app.preview.source_label.clone()),
                                ),
                        )
                        .children(query_lines.into_iter().enumerate().map(|(ix, line)| {
                            let color = if ix == 0 {
                                rgb(BLUE)
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
