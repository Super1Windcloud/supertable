use gpui::{
    AppContext, Context, Entity, IntoElement, Render, Window, div, prelude::*, rgb,
};
use gpui_component::{ActiveTheme, input::InputState};

use crate::{data::{Connection, load_connections}, palette::APP_BG};

use super::{editor, onboarding, results, sidebar, top_bar};

pub struct SuperTableApp {
    pub connections: Vec<Connection>,
    pub global_search: Entity<InputState>,
    pub grid_search: Entity<InputState>,
    pub onboarding_search: Entity<InputState>,
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

        Self {
            connections,
            global_search,
            grid_search,
            onboarding_search,
            selected_editor_tab: 0,
            selected_result_tab: 0,
        }
    }

    pub fn has_connections(&self) -> bool {
        !self.connections.is_empty()
    }
}

impl Render for SuperTableApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.has_connections() {
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
        }
    }
}
