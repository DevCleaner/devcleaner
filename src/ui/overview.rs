use human_bytes::human_bytes;
use tui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
};
use tui::layout::Alignment;
use tui::widgets::Wrap;

use crate::{app::App, banner::BANNER, Cli};
use crate::app::RouteId;
use crate::ui::control_panel::draw_control_panel_block;
use crate::ui::utils::{layout_block, style_help, title_style_logo};

use super::utils::{
    horizontal_chunks, style_default, style_logo, style_primary, vertical_chunks,
    vertical_chunks_with_margin,
};

pub fn draw_overview<B: Backend>(f: &mut Frame<B>, app: &mut App, cli: &Cli, area: Rect) {
    if app.show_info_bar {
        let chunks = vertical_chunks(vec![Constraint::Length(9), Constraint::Min(10)], area);
        draw_status_block(f, app, cli, chunks[0]);
        draw_control_panel_block(f, app, chunks[1]);
    } else {
        draw_control_panel_block(f, app, area);
    }
}

fn draw_status_block<B: Backend>(f: &mut Frame<B>, app: &mut App, cli: &Cli, area: Rect) {
    let chunks = horizontal_chunks(vec![Constraint::Min(10), Constraint::Length(50)], area);

    draw_general_info_block(f, app, cli, chunks[0]);
    draw_logo_block(f, app, chunks[1]);
}

fn draw_logo_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // Banner text with correct styling
    let text = format!(
        "{}\nv{} with ♥ in Rust {}",
        BANNER,
        env!("CARGO_PKG_VERSION"),
        nw_loading_indicator(app.is_loading)
    );
    let mut text = Text::from(text);
    text.patch_style(style_logo());

    // Contains the banner
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_general_info_block<B: Backend>(f: &mut Frame<B>, app: &mut App, cli: &Cli, area: Rect) {
    let chunks = vertical_chunks_with_margin(
        vec![
            Constraint::Length(4),
            Constraint::Min(2),
            Constraint::Min(2),
        ],
        area,
        1,
    );

    let block = layout_block(title_style_logo(app.title));

    f.render_widget(block, area);
    //  iterate over all  projects and create a text:Vec<Span> with the project count and the sum of all project sizes
    let mut project_count: i32 = 0;
    let mut project_size: u64 = 0;
    for project in &app.projects.items {
        project_count += 1;
        project_size += project.size;
    }
    let text = vec![
        Spans::from(vec![
            Span::styled("Path: ", style_default(app.light_theme)),
            Span::styled(cli.path.clone(), style_primary()),
        ]),
        Spans::from(vec![
            Span::styled("Criteria: ", style_default(app.light_theme)),
            Span::styled(cli.criteria.clone(), style_primary()),
        ]),
        Spans::from(vec![
            Span::styled("Projects: ", style_default(app.light_theme)),
            Span::styled(project_count.to_string(), style_primary()),
        ]),
        Spans::from(vec![
            Span::styled("Size: ", style_default(app.light_theme)),
            Span::styled(human_bytes(project_size as f64), style_primary()),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(Block::default());
    f.render_widget(paragraph, chunks[0]);

    draw_header_text(f, app, chunks[0])
}

fn draw_header_text<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match app.get_current_route().id {
        RouteId::Home => vec![Spans::from(
            "<up|down>: scroll context | <enter>: select context | <?> more help",
        )],
        _ => vec![Spans::from("<?> more help")],
    };
    let paragraph = Paragraph::new(text)
        .style(style_help())
        .block(Block::default())
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
// Utility methods

fn nw_loading_indicator<'a>(loading: bool) -> &'a str {
    if loading {
        "..."
    } else {
        ""
    }
}
