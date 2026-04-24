use gpui::{
    AppContext, Context, Entity, IntoElement, Render, Window, div, prelude::*, rgb,
};
use gpui_component::{ActiveTheme, input::InputState};

use crate::{
    data::{Connection, ConnectionKind, load_connections, save_connections},
    db::{DataPreview, load_preview},
    i18n::Locale,
    palette::{APP_BG, APP_BG_ALT},
};

use super::{connection_form, results, sidebar};

pub struct SuperTableApp {
    pub connections: Vec<Connection>,
    pub locale: Locale,
    pub preview: DataPreview,
    pub preview_error: Option<String>,
    pub connection_name: Entity<InputState>,
    pub connection_host: Entity<InputState>,
    pub connection_port: Entity<InputState>,
    pub connection_database: Entity<InputState>,
    pub connection_username: Entity<InputState>,
    pub connection_password: Entity<InputState>,
    pub connection_file_path: Entity<InputState>,
    pub show_connection_form: bool,
    pub selected_connection_kind: ConnectionKind,
}

impl SuperTableApp {
    pub fn new(window: &mut Window, cx: &mut Context<SuperTableApp>) -> Self {
        let locale = Locale::ZhCn;
        let connections = load_connections();
        let (preview, preview_error) = preview_for_active_connection(&connections);

        Self {
            connections,
            locale,
            preview,
            preview_error,
            connection_name: Self::build_input(
                window,
                cx,
                locale.connection_name_placeholder(),
                "",
            ),
            connection_host: Self::build_input(window, cx, "127.0.0.1", "127.0.0.1"),
            connection_port: Self::build_input(window, cx, "3306", "3306"),
            connection_database: Self::build_input(
                window,
                cx,
                locale.connection_database_placeholder(),
                "",
            ),
            connection_username: Self::build_input(
                window,
                cx,
                locale.connection_username_placeholder(),
                "",
            ),
            connection_password: Self::build_input(
                window,
                cx,
                locale.connection_password_placeholder(),
                "",
            ),
            connection_file_path: Self::build_input(window, cx, "C:\\data\\app.db", ""),
            show_connection_form: false,
            selected_connection_kind: ConnectionKind::MySql,
        }
    }

    fn build_input(
        window: &mut Window,
        cx: &mut Context<Self>,
        placeholder: &'static str,
        value: &str,
    ) -> Entity<InputState> {
        let value = value.to_string();
        cx.new(|cx: &mut Context<InputState>| {
            let mut input = InputState::new(window, cx).placeholder(placeholder);
            if !value.is_empty() {
                input.set_value(value.clone(), window, cx);
            }
            input
        })
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

        for item in &mut self.connections {
            item.active = false;
        }

        let connection = Connection {
            kind,
            name: name.trim().to_string(),
            host: self.connection_host.read(cx).value().to_string(),
            port,
            database: self.connection_database.read(cx).value().to_string(),
            username: self.connection_username.read(cx).value().to_string(),
            password: self.connection_password.read(cx).value().to_string(),
            file_path: self.connection_file_path.read(cx).value().to_string(),
            active: true,
        };

        self.connections.push(connection);
        let _ = save_connections(&self.connections);
        self.reload_preview();
        self.close_connection_form();
        self.reset_connection_form(window, cx);
        cx.notify();
    }

    pub fn activate_connection(&mut self, index: usize, cx: &mut Context<Self>) {
        for (current_index, item) in self.connections.iter_mut().enumerate() {
            item.active = current_index == index;
        }
        let _ = save_connections(&self.connections);
        self.reload_preview();
        cx.notify();
    }

    pub fn refresh_preview(&mut self, cx: &mut Context<Self>) {
        self.reload_preview();
        cx.notify();
    }

    fn reload_preview(&mut self) {
        let (preview, preview_error) = preview_for_active_connection(&self.connections);
        self.preview = preview;
        self.preview_error = preview_error;
    }

    fn reset_connection_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.connection_name
            .update(cx, |input, cx| input.set_value("", window, cx));
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
        div()
            .size_full()
            .bg(rgb(APP_BG_ALT))
            .child(
                div()
                    .size_full()
                    .bg(rgb(APP_BG))
                    .text_color(cx.theme().foreground)
                    .flex()
                    .child(sidebar::render(self, cx))
                    .child(
                        div()
                            .flex_1()
                            .p_3()
                            .child(results::render_panel(self, cx)),
                    ),
            )
            .when(self.show_connection_form, |this| {
                this.child(connection_form::render(self, window, cx))
            })
    }
}

fn preview_for_active_connection(connections: &[Connection]) -> (DataPreview, Option<String>) {
    let Some(connection) = connections.iter().find(|item| item.active) else {
        return (DataPreview::default(), None);
    };

    match load_preview(connection) {
        Ok(preview) => (preview, None),
        Err(err) => (
            DataPreview {
                source_label: connection.endpoint(),
                ..DataPreview::default()
            },
            Some(err.to_string()),
        ),
    }
}
