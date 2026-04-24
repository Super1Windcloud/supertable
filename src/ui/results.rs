use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::button::{Button, ButtonVariants};

use crate::palette::{
    ACCENT, BLUE_SOFT, BORDER, BORDER_SOFT, PANEL_ELEVATED, ROW_ALT, ROW_SELECTED, TABLE_BG,
    TEXT, TEXT_FAINT, TEXT_MUTED,
};

use super::app::SuperTableApp;

pub fn render_panel(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .size_full()
        .rounded(px(18.))
        .bg(rgb(TABLE_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .flex()
        .flex_col()
        .child(render_header(app, cx))
        .child(render_summary(app))
        .child(render_table_header(app))
        .child(render_rows(app))
}

fn render_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;
    let source_label = if app.preview.source_label.is_empty() {
        locale.active_source().to_string()
    } else {
        app.preview.source_label.clone()
    };

    div()
        .h(px(68.))
        .px_5()
        .bg(rgb(PANEL_ELEVATED))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_color(rgb(TEXT)).child(source_label))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(if app.preview.status_label.is_empty() {
                            locale.result_stats().to_string()
                        } else {
                            app.preview.status_label.clone()
                        }),
                ),
        )
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
                    Button::new("refresh-table")
                        .ghost()
                        .label(locale.refresh())
                        .on_click(cx.listener(|app, _, _, cx| {
                            app.refresh_preview(cx);
                        })),
                ),
        )
}

fn render_summary(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;
    let object_count: usize = app.preview.schema_items.iter().map(|item| item.count).sum();

    div()
        .h(px(54.))
        .px_5()
        .bg(rgb(TABLE_BG))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .gap_4()
        .child(metric_card(locale.objects(), object_count))
        .child(metric_card(locale.columns_label(), app.preview.columns.len()))
        .child(metric_card(locale.rows_label(), app.preview.rows.len()))
}

fn metric_card(label: &'static str, value: usize) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .px_2()
                .py_1()
                .rounded(px(999.))
                .bg(rgb(ROW_SELECTED))
                .text_size(px(11.))
                .text_color(rgb(ACCENT))
                .child(format!("{}", value)),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(TEXT_FAINT))
                .child(label),
        )
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
        .child(div().w(px(56.)).child("#"))
        .children(app.preview.columns.iter().map(|column| {
            div().w(px(196.)).truncate().child(column.clone())
        }))
}

fn render_rows(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;

    if let Some(error) = &app.preview_error {
        return div()
            .flex_1()
            .p_5()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_color(rgb(TEXT)).child(locale.load_failed()))
            .child(
                div()
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
                .h(px(44.))
                .px_4()
                .flex()
                .items_center()
                .bg(bg)
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .text_color(rgb(TEXT))
                .child(
                    div()
                        .w(px(56.))
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(format!("{:02}", ix + 1)),
                )
                .children(row.iter().map(|cell| {
                    div()
                        .w(px(196.))
                        .truncate()
                        .text_color(rgb(TEXT_MUTED))
                        .child(cell.clone())
                }))
        }))
}
