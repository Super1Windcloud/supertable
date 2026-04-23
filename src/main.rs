use gpui::{
    App, Application, Bounds, Context, Entity, IntoElement, ParentElement, Render, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_component::{
    ActiveTheme, Root,
    badge::Badge,
    button::{Button, ButtonVariants},
    input::{Input, InputState},
    tab::{Tab, TabBar},
    theme::{Theme, ThemeMode},
};

const APP_BG: u32 = 0x111317;
const PANEL_BG: u32 = 0x171A1F;
const PANEL_ELEVATED: u32 = 0x1C2027;
const PANEL_MUTED: u32 = 0x20252D;
const BORDER: u32 = 0x2A313B;
const BORDER_SOFT: u32 = 0x222830;
const TEXT: u32 = 0xE8EDF3;
const TEXT_MUTED: u32 = 0x97A3B6;
const TEXT_FAINT: u32 = 0x667182;
const ACCENT: u32 = 0x2BC48A;
const ACCENT_SOFT: u32 = 0x173A31;
const WARNING: u32 = 0xF4B84A;
const DANGER: u32 = 0xE86868;
const BLUE: u32 = 0x4DA3FF;
const ROW_SELECTED: u32 = 0x162431;
const ROW_ALT: u32 = 0x141920;

const CONNECTIONS: [Connection; 5] = [
    Connection::new("Production", "mysql://prod-us-east", "12 schemas", true, "PROD"),
    Connection::new("Analytics", "postgres://warehouse-main", "8 schemas", false, "LIVE"),
    Connection::new("Staging", "mysql://staging-shanghai", "11 schemas", false, "STAGE"),
    Connection::new("Local Dev", "sqlite://workspace.db", "3 schemas", false, "LOCAL"),
    Connection::new("Redis Cache", "redis://cache-eu-1", "6 dbs", false, "CACHE"),
];

const SCHEMA_ITEMS: [SchemaItem; 8] = [
    SchemaItem::new("Tables", 23, true),
    SchemaItem::new("Views", 5, false),
    SchemaItem::new("Functions", 12, false),
    SchemaItem::new("Triggers", 7, false),
    SchemaItem::new("Users", 4, false),
    SchemaItem::new("Migrations", 18, false),
    SchemaItem::new("Favorites", 9, false),
    SchemaItem::new("Pins", 3, false),
];

const QUERY_LINES: [&str; 8] = [
    "SELECT",
    "    o.id,",
    "    o.customer_name,",
    "    o.status,",
    "    o.total_amount,",
    "    o.updated_at",
    "FROM orders o",
    "WHERE o.status IN ('paid', 'refunded') ORDER BY o.updated_at DESC LIMIT 200;",
];

const ROWS: [ResultRow; 8] = [
    ResultRow::new("84721", "Pixel Union", "paid", "$12,480.00", "3 items", "2026-04-23 18:02"),
    ResultRow::new("84720", "Acme Labs", "refunded", "$420.00", "1 item", "2026-04-23 17:48"),
    ResultRow::new("84719", "Northwind", "paid", "$2,190.40", "9 items", "2026-04-23 17:30"),
    ResultRow::new("84718", "Neon Works", "paid", "$98.00", "1 item", "2026-04-23 17:08"),
    ResultRow::new("84717", "Delta Freight", "pending", "$4,800.00", "2 items", "2026-04-23 16:54"),
    ResultRow::new("84716", "Sora Health", "paid", "$1,244.10", "5 items", "2026-04-23 16:15"),
    ResultRow::new("84715", "Blue Harbor", "cancelled", "$640.00", "2 items", "2026-04-23 15:49"),
    ResultRow::new("84714", "Orbit Studio", "paid", "$8,020.00", "4 items", "2026-04-23 15:20"),
];

#[derive(Clone, Copy)]
struct Connection {
    name: &'static str,
    endpoint: &'static str,
    meta: &'static str,
    active: bool,
    badge: &'static str,
}

impl Connection {
    const fn new(
        name: &'static str,
        endpoint: &'static str,
        meta: &'static str,
        active: bool,
        badge: &'static str,
    ) -> Self {
        Self {
            name,
            endpoint,
            meta,
            active,
            badge,
        }
    }
}

#[derive(Clone, Copy)]
struct SchemaItem {
    name: &'static str,
    count: usize,
    active: bool,
}

impl SchemaItem {
    const fn new(name: &'static str, count: usize, active: bool) -> Self {
        Self {
            name,
            count,
            active,
        }
    }
}

#[derive(Clone, Copy)]
struct ResultRow {
    id: &'static str,
    customer: &'static str,
    status: &'static str,
    amount: &'static str,
    items: &'static str,
    updated_at: &'static str,
}

impl ResultRow {
    const fn new(
        id: &'static str,
        customer: &'static str,
        status: &'static str,
        amount: &'static str,
        items: &'static str,
        updated_at: &'static str,
    ) -> Self {
        Self {
            id,
            customer,
            status,
            amount,
            items,
            updated_at,
        }
    }
}

struct SuperTableApp {
    global_search: Entity<InputState>,
    grid_search: Entity<InputState>,
    selected_editor_tab: usize,
    selected_result_tab: usize,
}

impl SuperTableApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let global_search = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Search connections, tables, snippets")
        });
        let grid_search =
            cx.new(|cx| InputState::new(window, cx).placeholder("Filter 234 rows in result set"));

        Self {
            global_search,
            grid_search,
            selected_editor_tab: 0,
            selected_result_tab: 0,
        }
    }

    fn render_top_bar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(56.))
            .px_4()
            .flex()
            .items_center()
            .justify_between()
            .bg(rgb(PANEL_BG))
            .border_b_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .size(px(28.))
                            .rounded(px(8.))
                            .bg(rgb(ACCENT))
                            .text_color(rgb(0x08110D))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child("T"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_color(rgb(TEXT)).child("SuperTable"))
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(rgb(TEXT_FAINT))
                                    .child("TablePlus-inspired workspace"),
                            ),
                    ),
            )
            .child(
                div()
                    .w(px(420.))
                    .child(Input::new(&self.global_search).cleanable(true)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(Button::new("new-query").primary().label("New Query"))
                    .child(Button::new("import").ghost().label("Import"))
                    .child(Button::new("share").ghost().label("Share")),
            )
    }

    fn render_sidebar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
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
                                    .child("5 active endpoints"),
                            ),
                    )
                    .child(Button::new("add-conn").ghost().label("+ Add")),
            )
            .child(
                div()
                    .px_2()
                    .py_2()
                    .gap_2()
                    .flex()
                    .flex_col()
                    .children(CONNECTIONS.into_iter().map(|item| {
                        let bg = if item.active { rgb(PANEL_MUTED) } else { rgb(PANEL_BG) };
                        let border = if item.active { rgb(ACCENT) } else { rgb(BORDER_SOFT) };
                        let badge_color = if item.badge == "PROD" {
                            rgb(DANGER)
                        } else if item.badge == "LIVE" {
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
                                    .child(div().text_color(rgb(TEXT)).child(item.name))
                                    .child(
                                        div()
                                            .text_size(px(12.))
                                            .text_color(rgb(TEXT_MUTED))
                                            .child(item.endpoint),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.))
                                            .text_color(rgb(TEXT_FAINT))
                                            .child(item.meta),
                                    ),
                            )
                            .child(
                                Badge::new()
                                    .count(1)
                                    .color(badge_color)
                                    .child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded(px(999.))
                                            .bg(rgb(0x0F1318))
                                            .text_size(px(11.))
                                            .text_color(rgb(TEXT))
                                            .child(item.badge),
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

    fn render_editor_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        TabBar::new("editor-tabs")
            .underline()
            .selected_index(self.selected_editor_tab)
            .on_click(move |ix, _, cx| {
                entity.update(cx, |this, cx| {
                    this.selected_editor_tab = *ix;
                    cx.notify();
                });
            })
            .child(Tab::new().label("orders.sql"))
            .child(Tab::new().label("customers.sql"))
            .child(Tab::new().label("insights.sql"))
    }

    fn render_result_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        TabBar::new("result-tabs")
            .pill()
            .selected_index(self.selected_result_tab)
            .on_click(move |ix, _, cx| {
                entity.update(cx, |this, cx| {
                    this.selected_result_tab = *ix;
                    cx.notify();
                });
            })
            .child(Tab::new().label("Data"))
            .child(Tab::new().label("Structure"))
            .child(Tab::new().label("Console"))
    }

    fn render_sql_editor(&self) -> impl IntoElement {
        div()
            .flex_1()
            .rounded(px(14.))
            .bg(rgb(PANEL_BG))
            .border_1()
            .border_color(rgb(BORDER))
            .overflow_hidden()
            .child(
                div()
                    .h(px(42.))
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
                            .gap_2()
                            .child(Button::new("run").primary().label("Run"))
                            .child(Button::new("format").ghost().label("Format"))
                            .child(Button::new("explain").ghost().label("Explain")),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(TEXT_FAINT))
                            .child("Ctrl+Enter to execute selection"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .h_full()
                    .bg(rgb(0x101318))
                    .child(
                        div()
                            .w(px(52.))
                            .h_full()
                            .bg(rgb(0x0D1015))
                            .border_r_1()
                            .border_color(rgb(BORDER_SOFT))
                            .pt_3()
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
                            .pt_3()
                            .px_4()
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

    fn render_table_header(&self) -> impl IntoElement {
        div()
            .h(px(42.))
            .px_3()
            .bg(rgb(PANEL_ELEVATED))
            .border_b_1()
            .border_color(rgb(BORDER_SOFT))
            .flex()
            .items_center()
            .text_size(px(12.))
            .text_color(rgb(TEXT_FAINT))
            .child(div().w(px(86.)).child("ID"))
            .child(div().w(px(220.)).child("CUSTOMER"))
            .child(div().w(px(120.)).child("STATUS"))
            .child(div().w(px(120.)).child("AMOUNT"))
            .child(div().w(px(120.)).child("ITEMS"))
            .child(div().flex_1().child("UPDATED AT"))
    }

    fn render_status_pill(&self, status: &str) -> impl IntoElement {
        let (bg, fg) = match status {
            "paid" => (rgb(ACCENT_SOFT), rgb(ACCENT)),
            "refunded" => (rgb(0x362A18), rgb(WARNING)),
            "pending" => (rgb(0x2B2338), rgb(0xC291FF)),
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
            .child(status.to_owned())
    }

    fn render_result_row(&self, row: ResultRow, ix: usize) -> impl IntoElement {
        let bg = if ix == 0 {
            rgb(ROW_SELECTED)
        } else if ix % 2 == 0 {
            rgb(PANEL_BG)
        } else {
            rgb(ROW_ALT)
        };

        div()
            .h(px(46.))
            .px_3()
            .flex()
            .items_center()
            .bg(bg)
            .border_b_1()
            .border_color(rgb(BORDER_SOFT))
            .text_color(rgb(TEXT))
            .child(div().w(px(86.)).child(row.id))
            .child(div().w(px(220.)).child(row.customer))
            .child(div().w(px(120.)).child(self.render_status_pill(row.status)))
            .child(div().w(px(120.)).child(row.amount))
            .child(div().w(px(120.)).text_color(rgb(TEXT_MUTED)).child(row.items))
            .child(div().flex_1().text_color(rgb(TEXT_MUTED)).child(row.updated_at))
    }

    fn render_results_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .rounded(px(14.))
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
                    .child(self.render_result_tabs(cx))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w(px(240.))
                                    .child(Input::new(&self.grid_search).cleanable(true)),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(rgb(TEXT_FAINT))
                                    .child("234 rows • 18 ms"),
                            ),
                    ),
            )
            .child(self.render_table_header())
            .child(
                div()
                    .flex_1()
                    .children(ROWS.into_iter().enumerate().map(|(ix, row)| {
                        self.render_result_row(row, ix)
                    })),
            )
    }
}

impl Render for SuperTableApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(APP_BG))
            .text_color(cx.theme().foreground)
            .flex()
            .flex_col()
            .child(self.render_top_bar(cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .child(self.render_sidebar(cx))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .p_3()
                            .gap_3()
                            .child(
                                div()
                                    .rounded(px(14.))
                                    .bg(rgb(PANEL_BG))
                                    .border_1()
                                    .border_color(rgb(BORDER))
                                    .overflow_hidden()
                                    .child(self.render_editor_tabs(cx)),
                            )
                            .child(self.render_sql_editor())
                            .child(self.render_results_panel(cx)),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);

        let bounds = Bounds::centered(None, size(px(1440.), px(920.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| SuperTableApp::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
