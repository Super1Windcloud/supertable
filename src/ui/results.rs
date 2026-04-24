use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    button::{Button, ButtonVariants},
    input::Input,
    scroll::ScrollableElement,
};

use crate::palette::{
    ACCENT, BLUE_SOFT, BORDER, BORDER_SOFT, PANEL_ELEVATED, ROW_ALT, ROW_SELECTED, TABLE_BG,
    TEXT, TEXT_FAINT, TEXT_MUTED,
};

use super::app::SuperTableApp;

pub fn render_panel(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .size_full()
        .rounded(px(16.))
        .bg(rgb(TABLE_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .flex()
        .flex_col()
        .child(render_header(app, cx))
        .child(render_summary(app, cx))
        .child(render_table_header(app, cx))
        .child(render_rows(app, cx))
        .child(render_footer(app))
}

fn render_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;
    let source_label = if app.preview.source_label.is_empty() {
        locale.active_source().to_string()
    } else {
        app.preview.source_label.clone()
    };

    div()
        .h(px(46.))
        .px_4()
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
                .child(div().text_size(px(13.)).text_color(rgb(TEXT)).child(source_label))
                .child(
                    div()
                        .text_size(px(11.))
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
                .gap_2()
                .child(div().w(px(190.)).child(Input::new(&app.grid_filter).cleanable(true)))
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
                        .px_2()
                        .py_1()
                        .rounded(px(999.))
                        .bg(rgb(ROW_SELECTED))
                        .text_size(px(11.))
                        .text_color(rgb(ACCENT))
                        .child(locale.editable_grid()),
                )
                .when(app.editing_cell.is_some(), |this| {
                    this.child(
                        Button::new("save-cell-edit")
                            .ghost()
                            .label(locale.save())
                            .on_click(cx.listener(|app, _, _, cx| {
                                app.apply_cell_edit(cx);
                            })),
                    )
                    .child(
                        Button::new("cancel-cell-edit")
                            .ghost()
                            .label(locale.cancel_edit())
                            .on_click(cx.listener(|app, _, _, cx| {
                                app.cancel_cell_edit(cx);
                            })),
                    )
                })
                .when(app.dirty_cell.is_some(), |this| {
                    this.child(
                        Button::new("commit-grid-edit")
                            .ghost()
                            .label(locale.save())
                            .on_click(cx.listener(|app, _, _, cx| {
                                app.save_grid_changes(cx);
                            })),
                    )
                })
                .child(
                    Button::new("refresh-table")
                        .ghost()
                        .label(locale.refresh())
                        .on_click(cx.listener(|app, _, window, cx| {
                            app.refresh_preview(window, cx);
                        })),
                ),
        )
}

fn render_summary(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;
    let object_count: usize = app.preview.schema_items.iter().map(|item| item.count).sum();
    let visible_count = app.visible_row_indices(cx).len();

    div()
        .h(px(38.))
        .px_4()
        .bg(rgb(TABLE_BG))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .gap_4()
        .child(metric_card(locale.objects(), object_count))
        .child(metric_card(locale.columns_label(), app.preview.columns.len()))
        .child(metric_card(locale.rows_label(), visible_count))
        .child(metric_card(
            locale.selected_row(),
            app.selected_row.map(|row| row + 1).unwrap_or(0),
        ))
}

fn metric_card(label: &'static str, value: usize) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .px_2()
                .py_0p5()
                .rounded(px(999.))
                .bg(rgb(ROW_SELECTED))
                .text_size(px(11.))
                .text_color(rgb(ACCENT))
                .child(format!("{}", value)),
        )
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(label),
        )
}

fn render_table_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .h(px(36.))
        .px_4()
        .bg(rgb(PANEL_ELEVATED))
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .text_size(px(12.))
        .text_color(rgb(TEXT_FAINT))
        .child(div().w(px(56.)).child("#"))
        .children(app.preview.columns.iter().enumerate().map(|(index, column)| {
            let is_active_sort = app.sort_column == Some(index);
            let suffix = if is_active_sort {
                if app.sort_desc { " ↓" } else { " ↑" }
            } else {
                ""
            };

            div()
                .w(px(196.))
                .rounded(px(6.))
                .px_2()
                .py_0p5()
                .bg(if is_active_sort {
                    rgb(ROW_SELECTED)
                } else {
                    rgb(PANEL_ELEVATED)
                })
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |app, _, _, cx| {
                    app.toggle_sort(index, cx);
                }))
                .child(format!("{column}{suffix}"))
        }))
}

fn render_rows(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> gpui::AnyElement {
    let locale = app.locale;
    let visible_rows = app.visible_row_indices(cx);

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
            )
            .into_any_element();
    }

    if visible_rows.is_empty() {
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
            )
            .into_any_element();
    }

    div()
        .flex_1()
        .overflow_scrollbar()
        .children(visible_rows.into_iter().enumerate().map(|(visual_ix, row_index)| {
            let row = &app.preview.rows[row_index];
            let is_selected = app.selected_row == Some(row_index);
            let bg = if is_selected {
                rgb(ROW_SELECTED)
            } else if visual_ix % 2 == 0 {
                rgb(TABLE_BG)
            } else {
                rgb(ROW_ALT)
            };

            div()
                .h(px(38.))
                .px_4()
                .flex()
                .items_center()
                .bg(bg)
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .text_color(rgb(TEXT))
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |app, _, _, cx| {
                    app.select_row(row_index, cx);
                }))
                .child(
                    div()
                        .w(px(56.))
                        .text_size(px(12.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(format!("{:02}", row_index + 1)),
                )
                .children(row.iter().enumerate().map(|(col_ix, cell)| {
                    let is_editing = app.editing_cell == Some((row_index, col_ix));

                    div()
                        .w(px(196.))
                        .rounded(px(6.))
                        .px_2()
                        .py_0p5()
                        .bg(if is_editing { rgb(PANEL_ELEVATED) } else { bg })
                        .border_1()
                        .border_color(if is_editing {
                            rgb(ACCENT)
                        } else {
                            rgb(BORDER_SOFT)
                        })
                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(
                            move |app: &mut SuperTableApp, _, window, cx| {
                                app.begin_cell_edit(row_index, col_ix, window, cx);
                            },
                        ))
                        .child(if is_editing {
                            Input::new(&app.cell_editor).cleanable(true).into_any_element()
                        } else {
                            div()
                                .truncate()
                                .text_color(rgb(TEXT_MUTED))
                                .child(cell.clone())
                                .into_any_element()
                        })
                }))
        }))
        .into_any_element()
}

fn render_footer(app: &SuperTableApp) -> impl IntoElement {
    let locale = app.locale;

    div()
        .h(px(34.))
        .px_4()
        .bg(rgb(PANEL_ELEVATED))
        .border_t_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(format!(
                    "{} {} / {}",
                    locale.selected_row(),
                    app.selected_row.map(|row| row + 1).unwrap_or(0),
                    app.preview.rows.len()
                )),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(pager_chip(locale.first_page()))
                .child(pager_chip(locale.prev_page()))
                .child(pager_chip(locale.next_page()))
                .child(pager_chip(locale.last_page())),
        )
}

fn pager_chip(label: &'static str) -> impl IntoElement {
    div()
        .h(px(22.))
        .px_2()
        .rounded(px(6.))
        .bg(rgb(TABLE_BG))
        .border_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .text_size(px(11.))
        .text_color(rgb(TEXT_FAINT))
        .child(label)
}
