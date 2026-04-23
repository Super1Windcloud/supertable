use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    input::Input,
    tab::{Tab, TabBar},
};

use crate::palette::{
    BLUE_SOFT, BORDER, BORDER_SOFT, PANEL_ELEVATED, ROW_ALT, ROW_SELECTED, TABLE_BG, TEXT,
    TEXT_FAINT, TEXT_MUTED,
};

use super::app::SuperTableApp;

pub fn render_panel(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .flex_1()
        .rounded(px(18.))
        .bg(rgb(TABLE_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .child(render_toolbar(app, cx))
        .child(render_table_header(app))
        .child(render_rows(app))
}

fn render_toolbar(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .h(px(58.))
        .px_4()
        .flex()
        .items_center()
        .justify_between()
        .bg(rgb(PANEL_ELEVATED))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .child(render_tabs(app, cx))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(999.))
                        .bg(rgb(BLUE_SOFT))
                        .text_size(px(11.))
                        .text_color(rgb(TEXT))
                        .child(locale.live_source()),
                )
                .child(
                    div()
                        .w(px(240.))
                        .child(Input::new(&app.grid_search).cleanable(true)),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(format!("{} • {}", app.preview.rows.len(), locale.result_stats())),
                ),
        )
}

fn render_tabs(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;
    let entity = cx.entity().clone();
    TabBar::new("result-tabs")
        .pill()
        .selected_index(app.selected_result_tab)
        .on_click(move |ix, _, cx| {
            entity.update(cx, |this, cx| {
                this.selected_result_tab = *ix;
                cx.notify();
            });
        })
        .child(Tab::new().label(locale.data_tab()))
        .child(Tab::new().label(locale.structure_tab()))
        .child(Tab::new().label(locale.console_tab()))
}

fn render_table_header(app: &SuperTableApp) -> impl IntoElement {
    div()
        .h(px(42.))
        .px_4()
        .bg(rgb(PANEL_ELEVATED))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .text_size(px(12.))
        .text_color(rgb(TEXT_FAINT))
        .children(app.preview.columns.iter().map(|column| {
            div().w(px(180.)).truncate().child(column.clone())
        }))
}

fn render_rows(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;

    if let Some(error) = &app.preview_error {
        return div()
            .flex_1()
            .p_4()
            .child(div().text_color(rgb(TEXT)).child(locale.load_failed()))
            .child(
                div()
                    .mt_2()
                    .text_size(px(12.))
                    .text_color(rgb(TEXT_MUTED))
                    .child(error.clone()),
            );
    }

    if app.preview.rows.is_empty() {
        return div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(rgb(TEXT_MUTED))
                    .child(locale.no_data()),
            );
    }

    div()
        .flex_1()
        .children(app.preview.rows.iter().enumerate().map(|(ix, row)| {
            let bg = if ix == 0 {
                rgb(ROW_SELECTED)
            } else if ix % 2 == 0 {
                rgb(TABLE_BG)
            } else {
                rgb(ROW_ALT)
            };

            div()
                .h(px(48.))
                .px_4()
                .flex()
                .items_center()
                .bg(bg)
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .text_color(rgb(TEXT))
                .children(row.iter().map(|cell| {
                    div()
                        .w(px(180.))
                        .truncate()
                        .text_color(rgb(TEXT_MUTED))
                        .child(cell.clone())
                }))
        }))
}
