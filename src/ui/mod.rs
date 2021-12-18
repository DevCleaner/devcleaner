use tui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{App, RouteId};
use crate::Cli;

use self::{
    help::draw_help,
    overview::draw_overview,
    utils::{style_failure, style_main_background, style_primary, vertical_chunks},
};

mod control_panel;
mod help;
mod overview;
mod utils;

static HIGHLIGHT: &str = "=> ";

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, cli: &Cli) {
    let block = Block::default().style(style_main_background(app.light_theme));
    f.render_widget(block, f.size());

    let chunks = if !app.api_error.is_empty() {
        let chunks = vertical_chunks(
            vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ],
            f.size(),
        );
        draw_app_error(f, app, chunks[1]);
        chunks
    } else {
        vertical_chunks(vec![Constraint::Length(3)], f.size())
    };

    let last_chunk = chunks[chunks.len() - 1];
    match app.get_current_route().id {
        RouteId::HelpMenu => {
            draw_help(f, app, last_chunk);
        }
        _ => {
            draw_overview(f, app, cli, last_chunk);
        }
    }
}

fn draw_app_error<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect) {
    let block = Block::default()
        .title("Error | close <esc>")
        .style(style_failure())
        .borders(Borders::ALL);

    let mut text = Text::from(app.api_error.clone());
    text.patch_style(style_failure());

    let paragraph = Paragraph::new(text)
        .style(style_primary())
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, size);
}
