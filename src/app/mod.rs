use tui::layout::Rect;

use crate::app::project::Project;

use self::{
    key_binding::DEFAULT_KEYBINDING,
    models::{StatefulTable, TabRoute, TabsState},
};

pub(crate) mod key_binding;
pub(crate) mod models;
pub(crate) mod project;
pub(crate) mod utilities;

/// Holds main application state
pub struct App {
    navigation_stack: Vec<Route>,
    pub title: &'static str,
    pub should_quit: bool,
    pub main_tabs: TabsState,
    pub context_tabs: TabsState,
    pub show_info_bar: bool,
    pub is_loading: bool,
    pub is_streaming: bool,
    pub is_routing: bool,
    pub enhanced_graphics: bool,
    pub table_cols: u16,
    pub size: Rect,
    pub api_error: String,
    pub dialog: Option<String>,
    pub confirm: bool,
    pub light_theme: bool,
    pub refresh: bool,
    pub log_auto_scroll: bool,
    pub help_docs: StatefulTable<Vec<String>>,
    pub projects: StatefulTable<Project>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Help,
    Projects,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    HelpMenu,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Help,
};

impl Default for App {
    fn default() -> Self {
        App {
            navigation_stack: vec![DEFAULT_ROUTE],
            title: " NMKill - A dev tool for cleaning disk usage ",
            should_quit: false,
            main_tabs: TabsState::new(vec![TabRoute {
                title: format!(
                    "Active Context {}",
                    DEFAULT_KEYBINDING.jump_to_current_context.key
                ),
                route: Route {
                    active_block: ActiveBlock::Projects,
                    id: RouteId::Home,
                },
            }]),
            context_tabs: TabsState::new(vec![TabRoute {
                title: format!("Projects X {}", DEFAULT_KEYBINDING.jump_to_pods.key),
                route: Route {
                    active_block: ActiveBlock::Projects,
                    id: RouteId::Home,
                },
            }]),
            show_info_bar: true,
            is_loading: false,
            is_streaming: false,
            is_routing: false,
            enhanced_graphics: false,
            table_cols: 80,
            size: Rect::default(),
            api_error: String::new(),
            dialog: None,
            confirm: false,
            light_theme: false,
            refresh: false,
            log_auto_scroll: false,
            help_docs: StatefulTable::with_items(key_binding::get_help_docs()),
            projects: StatefulTable::new(),
        }
    }
}

impl App {
    pub fn new(projects: Vec<Project>, enhanced_graphics: bool) -> Self {
        App {
            projects: StatefulTable::with_items(projects),
            enhanced_graphics,
            ..App::default()
        }
    }
    // pub fn reset(&mut self) {
    //     self.api_error = String::new();
    //     self.projects = StatefulTable::new();
    //     self.route_home();
    // }
    // pub fn handle_error(&mut self, e: anyhow::Error) {
    //     self.api_error = e.to_string();
    // }
    pub fn push_navigation_stack(&mut self, id: RouteId, active_block: ActiveBlock) {
        self.push_navigation_route(Route { id, active_block });
    }
    pub fn push_navigation_route(&mut self, route: Route) {
        self.navigation_stack.push(route);
        self.is_routing = true;
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        self.is_routing = true;
        if self.navigation_stack.len() == 1 {
            None
        } else {
            self.navigation_stack.pop()
        }
    }

    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    // pub fn get_prev_route(&self) -> &Route {
    //     // get the previous route
    //     self.get_nth_route_from_last(1)
    // }
    // pub fn get_nth_route_from_last(&self, index: usize) -> &Route {
    //     // get the previous route by index
    //     let index = self.navigation_stack.len().saturating_sub(index + 1);
    //     if index > 0 {
    //         &self.navigation_stack[index]
    //     } else {
    //         &self.navigation_stack[0]
    //     }
    // }

    // pub fn route_home(&mut self) {
    //     let route = self.main_tabs.set_index(0).route.clone();
    //     self.push_navigation_route(route);
    // }
    // pub fn refresh(&mut self) {
    //     self.refresh = true;
    // }
}
