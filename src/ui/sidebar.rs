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
        .w(px(314.))
        .h_full()
        .bg(rgb(SIDEBAR_BG))
        .border_r_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .child(render_header(app, cx))
        .child(render_tree(app, cx))
}

fn render_header(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .border_b_1()
        .border_color(rgb(BORDER_SOFT))
        .child(
            div()
                .h(px(44.))
                .px_4()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(15.))
                        .text_color(rgb(TEXT))
                        .child(locale.database_explorer()),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(999.))
                        .bg(rgb(ACCENT_SOFT))
                        .text_size(px(10.))
                        .text_color(rgb(ACCENT))
                        .child("sqlx"),
                ),
        )
        .child(
            div()
                .h(px(40.))
                .px_3()
                .flex()
                .items_center()
                .gap_2()
                .text_size(px(11.))
                .text_color(rgb(TEXT_FAINT))
                .child(toolbar_chip("+", locale.create_connection()))
                .child(toolbar_chip("[]", locale.properties()))
                .child(toolbar_chip("R", locale.refresh()))
                .child(
                    div()
                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app, _, _, cx| {
                            app.deactivate_active_connection(cx);
                        }))
                        .child(toolbar_chip("o", locale.deactivate())),
                )
                .child(toolbar_chip("F", locale.filter()))
                .child(toolbar_chip("V", locale.view_options()))
                .child(
                    Button::new("add-connection-tree")
                        .ghost()
                        .label(locale.create_connection())
                        .on_click(cx.listener(|app, _, window, cx| {
                            app.open_connection_form(window, cx)
                        })),
                )
                .child(
                    Button::new("refresh-tree")
                        .ghost()
                        .label(locale.refresh())
                        .on_click(cx.listener(|app, _, window, cx| {
                            app.refresh_preview(window, cx);
                        })),
                ),
        )
}

fn toolbar_chip(icon: &'static str, label: &'static str) -> impl IntoElement {
    div()
        .h(px(22.))
        .px_2()
        .rounded(px(6.))
        .bg(rgb(SURFACE_SOFT))
        .border_1()
        .border_color(rgb(BORDER_SOFT))
        .flex()
        .items_center()
        .gap_1()
        .child(div().text_color(rgb(TEXT)).child(icon))
        .child(
            div()
                .text_size(px(10.))
                .text_color(rgb(TEXT_FAINT))
                .child(label),
        )
}

fn render_tree(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    let locale = app.locale;

    div()
        .flex_1()
        .px_3()
        .py_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(section_label(locale.my_databases()))
        .children(app.connections.iter().enumerate().map(|(index, item)| {
            let is_active = item.active;
            let badge_color = match item.kind {
                ConnectionKind::MySql => rgb(DANGER),
                ConnectionKind::PostgreSql | ConnectionKind::MongoDb => rgb(BLUE),
                _ => rgb(WARNING),
            };

            div()
                .flex()
                .flex_col()
                .rounded(px(10.))
                .bg(if is_active { rgb(PANEL_MUTED) } else { rgb(SIDEBAR_BG) })
                .border_1()
                .border_color(if is_active {
                    rgb(ACCENT)
                } else {
                    rgb(SIDEBAR_BG)
                })
                .child(
                    div()
                        .h(px(34.))
                        .px_2()
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |app, _, window, cx| {
                            app.activate_connection(index, window, cx);
                        }))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(tree_arrow(is_active))
                                .child(
                                    div()
                                        .text_color(if is_active { rgb(TEXT) } else { rgb(TEXT_MUTED) })
                                        .child(item.name.clone()),
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
                )
                .when(is_active, |this| {
                    this.child(
                        div()
                            .px_2()
                            .pb_2()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(tree_leaf(1, item.endpoint(), false))
                            .child(tree_leaf(
                                1,
                                if item.database.is_empty() {
                                    locale.schema_public().to_string()
                                } else {
                                    item.database.clone()
                                },
                                false,
                            ))
                            .child(render_active_object_group(app, cx)),
                    )
                })
        }))
}

fn render_active_object_group(app: &SuperTableApp, cx: &mut Context<SuperTableApp>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(tree_leaf(
            2,
            app.locale.schema_item("Tables"),
            false,
        ))
        .children(app.preview.object_names.iter().map(|name| {
            let active = app.preview.active_object.as_deref() == Some(name.as_str());
            div()
                .h(px(28.))
                .rounded(px(7.))
                .bg(if active { rgb(ACCENT_SOFT) } else { rgb(SIDEBAR_BG) })
                .border_1()
                .border_color(if active { rgb(ACCENT) } else { rgb(SIDEBAR_BG) })
                .px_2()
                .ml(px(34.))
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(gpui::MouseButton::Left, cx.listener({
                    let name = name.clone();
                    move |app, _, window, cx| {
                        app.select_object(name.clone(), window, cx);
                    }
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w(px(6.))
                                .h(px(6.))
                                .rounded(px(999.))
                                .bg(if active { rgb(ACCENT) } else { rgb(TEXT_FAINT) }),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(if active { rgb(TEXT) } else { rgb(TEXT_MUTED) })
                                .child(name.clone()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(rgb(TEXT_FAINT))
                        .child(if active { "open" } else { "" }),
                )
        }))
}

fn section_label(label: &'static str) -> impl IntoElement {
    div()
        .h(px(28.))
        .px_2()
        .flex()
        .items_center()
        .text_size(px(11.))
        .text_color(rgb(TEXT_FAINT))
        .child(label)
}

fn tree_arrow(open: bool) -> impl IntoElement {
    div()
        .w(px(10.))
        .text_size(px(10.))
        .text_color(rgb(TEXT_FAINT))
        .child(if open { "v" } else { ">" })
}

fn tree_leaf(indent: usize, label: String, strong: bool) -> impl IntoElement {
    div()
        .h(px(26.))
        .pl(px(14. * indent as f32))
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .w(px(6.))
                .h(px(6.))
                .rounded(px(999.))
                .bg(if strong { rgb(ACCENT) } else { rgb(TEXT_FAINT) }),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(if strong { rgb(TEXT) } else { rgb(TEXT_MUTED) })
                .child(label),
        )
}
