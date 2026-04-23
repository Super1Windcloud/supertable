use gpui::{Context, IntoElement, div, px, rgb, prelude::*};
use gpui_component::{
    input::Input,
    tab::{Tab, TabBar},
};

use crate::{
    data::{ROWS, ResultRow},
    palette::{
        ACCENT, ACCENT_SOFT, BLUE_SOFT, BORDER, BORDER_SOFT, DANGER, PANEL_ELEVATED,
        PANEL_MUTED, ROW_ALT, ROW_SELECTED, TABLE_BG, TEXT, TEXT_FAINT, TEXT_MUTED, WARNING,
    },
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
        .child(
            div()
                .flex_1()
                .children(
                    ROWS.into_iter()
                        .enumerate()
                        .map(|(ix, row)| render_result_row(app, row, ix)),
                ),
        )
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
                        .child(locale.sample()),
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
                        .child(locale.result_stats()),
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
    let locale = app.locale;

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
        .child(div().w(px(86.)).child("ID"))
        .child(div().w(px(220.)).child(locale.table_customer()))
        .child(div().w(px(120.)).child(locale.table_status()))
        .child(div().w(px(120.)).child(locale.table_amount()))
        .child(div().w(px(120.)).child(locale.table_items()))
        .child(div().flex_1().child(locale.table_updated_at()))
}

fn render_result_row(app: &SuperTableApp, row: ResultRow, ix: usize) -> impl IntoElement {
    let locale = app.locale;
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
        .child(div().w(px(86.)).child(row.id))
        .child(div().w(px(220.)).child(row.customer))
        .child(div().w(px(120.)).child(render_status_pill(locale.status(row.status), row.status)))
        .child(div().w(px(120.)).child(row.amount))
        .child(div().w(px(120.)).text_color(rgb(TEXT_MUTED)).child(row.items))
        .child(div().flex_1().text_color(rgb(TEXT_MUTED)).child(row.updated_at))
}

fn render_status_pill(label: String, status: &str) -> impl IntoElement {
    let (bg, fg) = match status {
        "paid" => (rgb(ACCENT_SOFT), rgb(ACCENT)),
        "refunded" => (rgb(0x362A18), rgb(WARNING)),
        "pending" => (rgb(0x29364A), rgb(0xAACDFF)),
        "cancelled" => (rgb(0x3A1D23), rgb(DANGER)),
        _ => (rgb(PANEL_MUTED), rgb(TEXT_MUTED)),
    };

    div()
        .px_2()
        .py_1()
        .rounded(px(999.))
        .bg(bg)
        .text_size(px(11.))
        .text_color(fg)
        .child(label)
}
