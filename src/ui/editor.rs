use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    input::Input,
    tab::{Tab, TabBar},
};

use crate::palette::{
    ACCENT, ACCENT_SOFT, BORDER, BORDER_SOFT, PANEL_BG, PANEL_ELEVATED, SURFACE_SOFT,
    TABLE_BG, TEXT, TEXT_FAINT, TEXT_MUTED,
};

use super::app::{EditorMode, SuperTableApp};

pub fn render_tabs(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let entity = cx.entity().clone();
    let tab_title = if app.preview.source_label.is_empty() {
        app.locale.query_console().to_string()
    } else {
        format!("{} [{}]", app.locale.query_console(), app.preview.source_label)
    };

    div()
        .h(px(46.))
        .rounded(px(16.))
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
                .child(Tab::new().label(tab_title))
                .child(Tab::new().label("inspect.sql"))
                .child(Tab::new().label("notes.sql")),
        )
}

pub fn render_sql_editor(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;
    div()
        .size_full()
        .rounded(px(16.))
        .bg(rgb(PANEL_BG))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(40.))
                .px_3()
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
                        .gap_2()
                        .child(
                            div()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app: &mut SuperTableApp, _, _, cx| {
                                    app.run_query_preview(cx);
                                }))
                                .child(action_chip(
                                    ">",
                                    locale.run(),
                                    app.editor_mode == EditorMode::Query,
                                )),
                        )
                        .child(
                            div()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app: &mut SuperTableApp, _, _, cx| {
                                    app.explain_current_query(cx);
                                }))
                                .child(action_chip(
                                    "O",
                                    locale.explain(),
                                    app.editor_mode == EditorMode::Explain,
                                )),
                        )
                        .child(
                            div()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app: &mut SuperTableApp, _, _, cx| {
                                    app.set_editor_mode(EditorMode::History, cx);
                                }))
                                .child(action_chip(
                                    "H",
                                    locale.history(),
                                    app.editor_mode == EditorMode::History,
                                )),
                        )
                        .child(
                            div()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app: &mut SuperTableApp, _, _, cx| {
                                    app.set_editor_mode(EditorMode::Params, cx);
                                }))
                                .child(action_chip(
                                    "P",
                                    locale.params(),
                                    app.editor_mode == EditorMode::Params,
                                )),
                        )
                        .child(
                            div()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app: &mut SuperTableApp, _, _, cx| {
                                    app.set_editor_mode(EditorMode::InEditorResults, cx);
                                }))
                                .child(action_chip(
                                    "[]",
                                    locale.in_editor_results(),
                                    app.editor_mode == EditorMode::InEditorResults,
                                )),
                        )
                        .child(toolbar_chip(locale.tx_auto()))
                        .child(toolbar_chip(locale.playground())),
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
                .flex_1()
                .flex()
                .bg(rgb(TABLE_BG))
                .child(
                    div()
                        .w(px(54.))
                        .h_full()
                        .bg(rgb(SURFACE_SOFT))
                        .border_r_1()
                        .border_color(rgb(BORDER_SOFT))
                        .child(div()),
                )
                .child(
                    div()
                        .flex_1()
                        .px_2()
                        .py_2()
                        .child(
                            div()
                                .mb_2()
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
                        .child(Input::new(&app.query_editor).cleanable(false))
                        .child(render_aux_panel(app)),
                ),
        )
}

fn render_aux_panel(app: &SuperTableApp) -> gpui::AnyElement {
    match app.editor_mode {
        EditorMode::Query => render_completion_popup(app),
        EditorMode::Explain => render_info_panel(
            "EXPLAIN",
            vec![
                "Seq Scan / Index Scan preview".to_string(),
                "Join strategy".to_string(),
                "Estimated cost".to_string(),
            ],
        )
        .into_any_element(),
        EditorMode::History => render_info_panel(
            "HISTORY",
            app.preview
                .query_lines
                .iter()
                .map(|line| format!("recent: {line}"))
                .collect(),
        )
        .into_any_element(),
        EditorMode::Params => render_info_panel(
            "PARAMS",
            vec![
                "limit = 50".to_string(),
                "schema = public".to_string(),
                "source = current connection".to_string(),
            ],
        )
        .into_any_element(),
        EditorMode::InEditorResults => render_info_panel(
            "RESULTS",
            vec![
                format!("rows: {}", app.preview.rows.len()),
                format!("columns: {}", app.preview.columns.len()),
                app.preview.status_label.clone(),
            ],
        )
        .into_any_element(),
    }
}

fn toolbar_chip(label: &'static str) -> impl IntoElement {
    div()
        .h(px(24.))
        .px_2()
        .rounded(px(7.))
        .bg(rgb(SURFACE_SOFT))
        .border_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .text_size(px(11.))
        .text_color(rgb(TEXT_FAINT))
        .child(label)
}

fn action_chip(icon: &'static str, label: &'static str, active: bool) -> impl IntoElement {
    div()
        .h(px(24.))
        .px_2()
        .rounded(px(7.))
        .bg(if active { rgb(ACCENT_SOFT) } else { rgb(SURFACE_SOFT) })
        .border_1()
        .border_color(if active { rgb(ACCENT) } else { rgb(BORDER_SOFT) })
        .flex()
        .items_center()
        .gap_1()
        .child(div().text_color(if active { rgb(ACCENT) } else { rgb(TEXT) }).child(icon))
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(label),
        )
}

fn render_completion_popup(app: &SuperTableApp) -> gpui::AnyElement {
    if app.preview.object_names.is_empty() {
        return div().into_any_element();
    }

    render_info_panel(
        "JOIN",
        app.preview
            .object_names
            .iter()
            .take(8)
            .map(|name| format!("JOIN {}", name))
            .collect(),
    )
    .into_any_element()
}

fn render_info_panel(title: &'static str, lines: Vec<String>) -> impl IntoElement {
    div()
        .mt_4()
        .w(px(520.))
        .rounded(px(12.))
        .bg(rgb(PANEL_ELEVATED))
        .border_1()
        .border_color(rgb(BORDER))
        .overflow_hidden()
        .child(
            div()
                .h(px(34.))
                .px_3()
                .bg(rgb(SURFACE_SOFT))
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .flex()
                .items_center()
                .text_size(px(12.))
                .text_color(rgb(TEXT))
                .child(title),
        )
        .children(lines.into_iter().enumerate().map(|(ix, line)| {
            div()
                .h(px(34.))
                .px_3()
                .flex()
                .items_center()
                .bg(if ix == 0 { rgb(ACCENT_SOFT) } else { rgb(PANEL_ELEVATED) })
                .border_b_1()
                .border_color(rgb(BORDER_SOFT))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(if ix == 0 { rgb(TEXT) } else { rgb(TEXT_MUTED) })
                        .child(line),
                )
        }))
}
