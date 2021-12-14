use crossterm::event::{MouseEvent, MouseEventKind};

use crate::operations::{delete_directories, delete_directory};
use crate::{
    app::{
        key_binding::DEFAULT_KEYBINDING,
        models::{Scrollable, StatefulTable},
        ActiveBlock, App, RouteId,
    },
    event::Key,
};

pub async fn handle_key_events(key: Key, app: &mut App) {
    // First handle any global event and then move to route event
    match key {
        _ if key == DEFAULT_KEYBINDING.esc.key => {
            handle_escape(app);
        }
        _ if key == DEFAULT_KEYBINDING.quit.key || key == DEFAULT_KEYBINDING.quit.alt.unwrap() => {
            app.should_quit = true;
        }
        _ if key == DEFAULT_KEYBINDING.up.key || key == DEFAULT_KEYBINDING.up.alt.unwrap() => {
            handle_block_scroll(app, true, false).await;
        }
        _ if key == DEFAULT_KEYBINDING.down.key || key == DEFAULT_KEYBINDING.down.alt.unwrap() => {
            handle_block_scroll(app, false, false).await;
        }
        _ if key == DEFAULT_KEYBINDING.pg_up.key => handle_route_events(key, app).await,
        _ if key == DEFAULT_KEYBINDING.pg_down.key => handle_route_events(key, app).await,
        _ if key == DEFAULT_KEYBINDING.toggle_theme.key => {
            app.light_theme = !app.light_theme;
        }
        _ if key == DEFAULT_KEYBINDING.help.key => {
            app.push_navigation_stack(RouteId::HelpMenu, ActiveBlock::Help)
        }
        _ if key == DEFAULT_KEYBINDING.jump_to_pods.key => {
            let route = app.context_tabs.set_index(0).route.clone();
            app.push_navigation_route(route);
        }
        _ if key == DEFAULT_KEYBINDING.jump_to_all_context.key => {
            delete_all_projects(app);
        }
        _ => handle_route_events(key, app).await,
    }
}

fn delete_all_projects(app: &mut App) {
    delete_directories(
        app.projects
            .items
            .iter()
            .map(|p| p.path.clone())
            .collect::<Vec<_>>(),
    );
    app.projects.items.clear();
}

pub async fn handle_mouse_events(mouse: MouseEvent, app: &mut App) {
    match mouse.kind {
        // mouse scrolling is inverted
        MouseEventKind::ScrollDown => handle_block_scroll(app, true, false).await,
        MouseEventKind::ScrollUp => handle_block_scroll(app, false, false).await,
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    // dismiss error
    if !app.api_error.is_empty() {
        app.api_error = String::default();
    }
    match app.get_current_route().id {
        RouteId::HelpMenu => {
            app.pop_navigation_stack();
        }
        RouteId::Home => {
            app.pop_navigation_stack();
        }
    }
}

// Handle event for the current active block
async fn handle_route_events(key: Key, app: &mut App) {
    let route = app.context_tabs.set_index(0).route.clone();
    app.push_navigation_route(route);
    // route specific events
    let item_option = handle_block_action(key, &mut app.projects);
    if let Some(item) = item_option {
        delete_directory(item.path.clone().as_os_str().to_str().to_owned().unwrap());
        // delete project app.projects.items
        app.projects.items.retain(|x| *x != item);
    }
}

fn handle_block_action<T: Clone>(key: Key, item: &mut StatefulTable<T>) -> Option<T> {
    match key {
        _ if key == DEFAULT_KEYBINDING.submit.key
            || key == DEFAULT_KEYBINDING.describe_resource.key
            || key == DEFAULT_KEYBINDING.resource_yaml.key =>
        {
            item.get_selected_item_copy()
        }
        _ => None,
    }
}

async fn handle_block_scroll(app: &mut App, up: bool, page: bool) {
    match app.get_current_route().active_block {
        ActiveBlock::Help => app.help_docs.handle_scroll(up, page),
        ActiveBlock::Projects => app.projects.handle_scroll(up, page),
    }
}
