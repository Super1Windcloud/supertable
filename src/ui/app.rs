use gpui::{
    AppContext, Context, Entity, IntoElement, Render, Window, div, prelude::*, rgb,
};
use gpui_component::{ActiveTheme, input::InputState};

use crate::{
    data::{Connection, ConnectionKind, load_connections, save_connections},
    palette::APP_BG,
};

use super::{connection_form, editor, onboarding, results, sidebar, top_bar};

pub struct SuperTableApp {
    pub connections: Vec<Connection>,
    pub global_search: Entity<InputState>,
    pub grid_search: Entity<InputState>,
    pub onboarding_search: Entity<InputState>,
    pub connection_name: Entity<InputState>,
    pub connection_host: Entity<InputState>,
    pub connection_port: Entity<InputState>,
    pub connection_database: Entity<InputState>,
    pub connection_username: Entity<InputState>,
    pub connection_password: Entity<InputState>,
    pub connection_file_path: Entity<InputState>,
    pub show_connection_form: bool,
    pub selected_connection_kind: ConnectionKind,
    pub selected_editor_tab: usize,
    pub selected_result_tab: usize,
}

impl SuperTableApp {
    pub fn new(window: &mut Window, cx: &mut Context<SuperTableApp>) -> Self {
        let connections = load_connections();
        let global_search: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("Search connections, tables, snippets")
        });
        let grid_search: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("Filter 234 rows in result set")
        });
        let onboarding_search: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("搜索连接... (Ctrl F)")
        });
        let connection_name: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("连接名称")
        });
        let connection_host: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("127.0.0.1")
        });
        let connection_port: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("3306")
        });
        let connection_database: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("数据库 / schema / db index")
        });
        let connection_username: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("用户名")
        });
        let connection_password: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("密码")
        });
        let connection_file_path: Entity<InputState> = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("C:\\data\\app.db")
        });

        Self {
            connections,
            global_search,
            grid_search,
            onboarding_search,
            connection_name,
            connection_host,
            connection_port,
            connection_database,
            connection_username,
            connection_password,
            connection_file_path,
            show_connection_form: false,
            selected_connection_kind: ConnectionKind::MySql,
            selected_editor_tab: 0,
            selected_result_tab: 0,
        }
    }

    pub fn has_connections(&self) -> bool {
        !self.connections.is_empty()
    }

    pub fn open_connection_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_connection_form = true;
        self.selected_connection_kind = ConnectionKind::MySql;
        self.reset_connection_form(window, cx);
    }

    pub fn close_connection_form(&mut self) {
        self.show_connection_form = false;
    }

    pub fn set_connection_kind(
        &mut self,
        kind: ConnectionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_connection_kind = kind;
        self.connection_port.update(cx, |input, cx| {
            let value = if kind == ConnectionKind::Sqlite {
                String::new()
            } else {
                kind.default_port().to_string()
            };
            input.set_value(value, window, cx);
        });
        cx.notify();
    }

    pub fn save_connection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.connection_name.read(cx).value().to_string();
        if name.trim().is_empty() {
            return;
        }

        let kind = self.selected_connection_kind;
        let port = self
            .connection_port
            .read(cx)
            .value()
            .parse::<u16>()
            .ok()
            .unwrap_or(kind.default_port());

        let connection = Connection {
            kind,
            name: name.trim().to_string(),
            host: self.connection_host.read(cx).value().to_string(),
            port,
            database: self.connection_database.read(cx).value().to_string(),
            username: self.connection_username.read(cx).value().to_string(),
            password: self.connection_password.read(cx).value().to_string(),
            file_path: self.connection_file_path.read(cx).value().to_string(),
            active: self.connections.is_empty(),
        };

        self.connections.push(connection);
        let _ = save_connections(&self.connections);
        self.close_connection_form();
        self.reset_connection_form(window, cx);
        cx.notify();
    }

    fn reset_connection_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.connection_name.update(cx, |input, cx| input.set_value("", window, cx));
        self.connection_host
            .update(cx, |input, cx| input.set_value("127.0.0.1", window, cx));
        self.connection_port.update(cx, |input, cx| {
            input.set_value(self.selected_connection_kind.default_port().to_string(), window, cx)
        });
        self.connection_database
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.connection_username
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.connection_password
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.connection_file_path
            .update(cx, |input, cx| input.set_value("", window, cx));
    }
}

impl Render for SuperTableApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = if self.has_connections() {
            div()
                .size_full()
                .bg(rgb(APP_BG))
                .text_color(cx.theme().foreground)
                .flex()
                .flex_col()
                .child(top_bar::render(self, cx))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .child(sidebar::render(self, cx))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .p_3()
                                .gap_3()
                                .child(editor::render_tabs(self, cx))
                                .child(editor::render_sql_editor())
                                .child(results::render_panel(self, cx)),
                        ),
                )
                .into_any_element()
        } else {
            onboarding::render(self, cx).into_any_element()
        };

        div()
            .size_full()
            .child(body)
            .when(self.show_connection_form, |this| {
                this.child(connection_form::render(self, window, cx))
            })
    }
}
