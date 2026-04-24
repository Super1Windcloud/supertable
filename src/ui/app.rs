use gpui::{
    App, AppContext, Context, Entity, IntoElement, Render, Window, div, prelude::*, rgb,
};
use gpui_component::{ActiveTheme, input::InputState};

use crate::{
    data::{Connection, ConnectionKind, load_connections, save_connections},
    db::{DataPreview, execute_query, explain_query, load_preview, save_cell_edit},
    i18n::Locale,
    palette::{APP_BG, APP_BG_ALT},
};

use super::{connection_form, editor, results, sidebar};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum EditorMode {
    Query,
    Explain,
    History,
    Params,
    InEditorResults,
}

pub struct SuperTableApp {
    pub connections: Vec<Connection>,
    pub locale: Locale,
    pub preview: DataPreview,
    pub preview_error: Option<String>,
    pub grid_filter: Entity<InputState>,
    pub query_editor: Entity<InputState>,
    pub cell_editor: Entity<InputState>,
    pub editing_cell: Option<(usize, usize)>,
    pub dirty_cell: Option<(usize, usize)>,
    pub selected_row: Option<usize>,
    pub sort_column: Option<usize>,
    pub sort_desc: bool,
    pub selected_object: Option<String>,
    pub selected_editor_tab: usize,
    pub editor_mode: EditorMode,
    pub query_history: Vec<String>,
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
        let (preview, preview_error) = preview_for_active_connection(&connections, None);
        let initial_query = preview.query_lines.join("\n");

        Self {
            connections,
            locale,
            preview,
            preview_error,
            grid_filter: Self::build_input(window, cx, locale.filter_rows(), ""),
            query_editor: Self::build_code_editor(window, cx, &initial_query),
            cell_editor: Self::build_input(window, cx, "", ""),
            editing_cell: None,
            dirty_cell: None,
            selected_row: None,
            sort_column: None,
            sort_desc: false,
            selected_object: None,
            selected_editor_tab: 0,
            editor_mode: EditorMode::Query,
            query_history: Vec::new(),
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

    fn build_code_editor(
        window: &mut Window,
        cx: &mut Context<Self>,
        value: &str,
    ) -> Entity<InputState> {
        let value = value.to_string();
        cx.new(|cx: &mut Context<InputState>| {
            let mut input = InputState::new(window, cx)
                .code_editor("sql")
                .rows(14)
                .line_number(true);
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
        self.selected_object = None;
        self.reload_preview(window, cx);
        self.close_connection_form();
        self.reset_connection_form(window, cx);
        cx.notify();
    }

    pub fn activate_connection(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        for (current_index, item) in self.connections.iter_mut().enumerate() {
            item.active = current_index == index;
        }
        let _ = save_connections(&self.connections);
        self.selected_object = None;
        self.reload_preview(window, cx);
        cx.notify();
    }

    pub fn refresh_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reload_preview(window, cx);
        cx.notify();
    }

    pub fn deactivate_active_connection(&mut self, cx: &mut Context<Self>) {
        for item in &mut self.connections {
            item.active = false;
        }
        let _ = save_connections(&self.connections);
        self.preview = DataPreview::default();
        self.preview_error = None;
        self.selected_object = None;
        self.selected_row = None;
        self.dirty_cell = None;
        self.editing_cell = None;
        cx.notify();
    }

    pub fn select_object(&mut self, name: String, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_object = Some(name);
        self.reload_preview(window, cx);
        cx.notify();
    }

    pub fn toggle_sort(&mut self, column: usize, cx: &mut Context<Self>) {
        if self.sort_column == Some(column) {
            self.sort_desc = !self.sort_desc;
        } else {
            self.sort_column = Some(column);
            self.sort_desc = false;
        }
        cx.notify();
    }

    pub fn select_row(&mut self, row: usize, cx: &mut Context<Self>) {
        self.selected_row = Some(row);
        cx.notify();
    }

    pub fn set_editor_mode(&mut self, mode: EditorMode, cx: &mut Context<Self>) {
        self.editor_mode = mode;
        cx.notify();
    }

    pub fn run_query_preview(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.connections.iter().find(|item| item.active).cloned() else {
            self.preview_error = Some("No active connection".to_string());
            cx.notify();
            return;
        };
        let query = self.query_editor.read(cx).value().trim().to_string();
        if query.is_empty() {
            self.preview_error = Some("Query is empty".to_string());
            cx.notify();
            return;
        }
        match execute_query(&connection, &query) {
            Ok(preview) => {
                self.preview = preview;
                self.preview_error = None;
                self.editor_mode = EditorMode::Query;
                self.query_history.insert(0, query);
                self.query_history.truncate(20);
            }
            Err(err) => {
                self.preview_error = Some(err.to_string());
            }
        }
        cx.notify();
    }

    pub fn explain_current_query(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.connections.iter().find(|item| item.active).cloned() else {
            self.preview_error = Some("No active connection".to_string());
            cx.notify();
            return;
        };
        let query = self.query_editor.read(cx).value().trim().to_string();
        if query.is_empty() {
            self.preview_error = Some("Query is empty".to_string());
            cx.notify();
            return;
        }
        match explain_query(&connection, &query) {
            Ok(preview) => {
                self.preview = preview;
                self.preview_error = None;
                self.editor_mode = EditorMode::Explain;
            }
            Err(err) => {
                self.preview_error = Some(err.to_string());
            }
        }
        cx.notify();
    }

    pub fn visible_row_indices(&self, cx: &App) -> Vec<usize> {
        let filter = self.grid_filter.read(cx).value().trim().to_lowercase();
        let mut indices = self
            .preview
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                if filter.is_empty() {
                    true
                } else {
                    row.iter().any(|cell| cell.to_lowercase().contains(&filter))
                }
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        if let Some(column) = self.sort_column {
            indices.sort_by(|left, right| {
                let left_value = self
                    .preview
                    .rows
                    .get(*left)
                    .and_then(|row| row.get(column))
                    .cloned()
                    .unwrap_or_default();
                let right_value = self
                    .preview
                    .rows
                    .get(*right)
                    .and_then(|row| row.get(column))
                    .cloned()
                    .unwrap_or_default();
                if self.sort_desc {
                    right_value.cmp(&left_value)
                } else {
                    left_value.cmp(&right_value)
                }
            });
        }

        indices
    }

    pub fn begin_cell_edit(
        &mut self,
        row: usize,
        column: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(value) = self
            .preview
            .rows
            .get(row)
            .and_then(|cells| cells.get(column))
            .cloned()
        else {
            return;
        };

        self.cell_editor
            .update(cx, |input, cx| input.set_value(value, window, cx));
        self.editing_cell = Some((row, column));
        cx.notify();
    }

    pub fn apply_cell_edit(&mut self, cx: &mut Context<Self>) {
        let Some((row, column)) = self.editing_cell else {
            return;
        };
        let value = self.cell_editor.read(cx).value().to_string();
        if let Some(cell) = self
            .preview
            .rows
            .get_mut(row)
            .and_then(|cells| cells.get_mut(column))
        {
            *cell = value;
        }
        self.editing_cell = None;
        self.dirty_cell = Some((row, column));
        cx.notify();
    }

    pub fn cancel_cell_edit(&mut self, cx: &mut Context<Self>) {
        self.editing_cell = None;
        cx.notify();
    }

    pub fn save_grid_changes(&mut self, cx: &mut Context<Self>) {
        let Some((row, column)) = self.dirty_cell else {
            return;
        };
        let Some(connection) = self.connections.iter().find(|item| item.active).cloned() else {
            self.preview_error = Some("No active connection".to_string());
            cx.notify();
            return;
        };
        let Some(new_value) = self
            .preview
            .rows
            .get(row)
            .and_then(|cells| cells.get(column))
            .cloned()
        else {
            self.preview_error = Some("Edited cell was not found".to_string());
            cx.notify();
            return;
        };

        match save_cell_edit(&connection, &self.preview, row, column, &new_value) {
            Ok(()) => {
                match &mut self.preview.binding {
                    crate::db::PreviewBinding::SqlTable { original_rows, .. } => {
                        if let Some(cell) = original_rows
                            .get_mut(row)
                            .and_then(|cells| cells.get_mut(column))
                        {
                            *cell = new_value;
                        }
                    }
                    crate::db::PreviewBinding::RedisKeys { .. }
                    | crate::db::PreviewBinding::MongoCollection { .. }
                    | crate::db::PreviewBinding::None => {}
                }
                self.preview_error = None;
                self.dirty_cell = None;
            }
            Err(err) => {
                self.preview_error = Some(err.to_string());
            }
        }

        cx.notify();
    }

    fn reload_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (preview, preview_error) =
            preview_for_active_connection(&self.connections, self.selected_object.as_deref());
        self.preview = preview;
        self.preview_error = preview_error;
        self.query_editor
            .update(cx, |input, cx| input.set_value(self.preview.query_lines.join("\n"), window, cx));
        self.editing_cell = None;
        self.dirty_cell = None;
        self.selected_row = None;
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
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(editor::render_tabs(self, cx))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(div().flex_1().child(editor::render_sql_editor(self, cx)))
                                    .child(div().flex_1().child(results::render_panel(self, cx))),
                            ),
                    ),
            )
            .when(self.show_connection_form, |this| {
                this.child(connection_form::render(self, window, cx))
            })
    }
}

fn preview_for_active_connection(
    connections: &[Connection],
    selected_object: Option<&str>,
) -> (DataPreview, Option<String>) {
    let Some(connection) = connections.iter().find(|item| item.active) else {
        return (DataPreview::default(), None);
    };

    match load_preview(connection, selected_object) {
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
