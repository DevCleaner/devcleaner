use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Row},
    Frame,
};
// Utils

const DARK_FG_COLOR: Color = Color::White;
const DARK_BG_COLOR: Color = Color::Rgb(35, 50, 55);
const LIGHT_FG_COLOR: Color = Color::Magenta;
const LIGHT_BG_COLOR: Color = Color::White;

pub fn title_style(txt: &str) -> Span {
    Span::styled(txt, style_bold())
}

pub fn title_style_logo(txt: &str) -> Span {
    Span::styled(
        txt,
        style_logo()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC),
    )
}

pub fn style_bold() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

pub fn style_default(light: bool) -> Style {
    if light {
        Style::default().fg(LIGHT_FG_COLOR)
    } else {
        Style::default().fg(DARK_FG_COLOR)
    }
}
pub fn style_logo() -> Style {
    Style::default().fg(Color::Blue)
}
pub fn style_failure() -> Style {
    Style::default().fg(Color::Red)
}
pub fn style_success() -> Style {
    Style::default().fg(Color::Green)
}
pub fn style_highlight() -> Style {
    Style::default().add_modifier(Modifier::REVERSED)
}
pub fn style_primary() -> Style {
    Style::default().fg(Color::Cyan)
}
pub fn style_help() -> Style {
    Style::default().fg(Color::Blue)
}

pub fn style_secondary() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn style_main_background(light: bool) -> Style {
    match light {
        true => Style::default().bg(LIGHT_BG_COLOR).fg(LIGHT_FG_COLOR),
        false => Style::default().bg(DARK_BG_COLOR).fg(DARK_FG_COLOR),
    }
}

pub fn table_header_style(cells: Vec<&str>, light: bool) -> Row {
    Row::new(cells).style(style_default(light)).bottom_margin(0)
}

pub fn horizontal_chunks(constraints: Vec<Constraint>, size: Rect) -> Vec<Rect> {
    Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Horizontal)
        .split(size)
}

pub fn vertical_chunks(constraints: Vec<Constraint>, size: Rect) -> Vec<Rect> {
    Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Vertical)
        .split(size)
}

pub fn vertical_chunks_with_margin(
    constraints: Vec<Constraint>,
    size: Rect,
    margin: u16,
) -> Vec<Rect> {
    Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Vertical)
        .margin(margin)
        .split(size)
}

pub fn layout_block(title: Span) -> Block {
    Block::default().borders(Borders::ALL).title(title)
}

pub fn layout_block_default(title: &str) -> Block {
    layout_block(title_style(title))
}

pub fn layout_block_active_span(title: Spans) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(style_secondary())
}

pub fn layout_block_top_border(title: Spans) -> Block {
    Block::default().borders(Borders::TOP).title(title)
}

pub fn title_with_dual_style<'a>(part_1: String, part_2: String, light: bool) -> Spans<'a> {
    Spans::from(vec![
        Span::styled(part_1, style_secondary().add_modifier(Modifier::BOLD)),
        Span::styled(part_2, style_default(light).add_modifier(Modifier::BOLD)),
    ])
}

pub fn loading<B: Backend>(f: &mut Frame<B>, block: Block, area: Rect, is_loading: bool) {
    if is_loading {
        let text = "\n\n Loading ...\n\n".to_owned();
        let mut text = Text::from(text);
        text.patch_style(style_secondary());

        // Contains the text
        let paragraph = Paragraph::new(text).style(style_secondary()).block(block);
        f.render_widget(paragraph, area);
    } else {
        f.render_widget(block, area)
    }
}
