use super::{
    utils::{
        layout_block_default, layout_block_top_border, loading, style_default, style_highlight,
        style_primary, style_secondary, style_success, table_header_style, title_with_dual_style,
        vertical_chunks_with_margin,
    },
    HIGHLIGHT,
};
use crate::app::{models::StatefulTable, App};
use human_bytes::human_bytes;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::Style,
    text::{Span, Spans},
    widgets::{Cell, Row, Table, Tabs},
    Frame,
};
pub fn draw_control_panel_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks =
        vertical_chunks_with_margin(vec![Constraint::Length(2), Constraint::Min(0)], area, 1);
    let block = layout_block_default("Resources");

    let tabs = Tabs::new(vec![Spans::from(Span::styled(
        "Acosta",
        style_default(app.light_theme),
    ))])
    .block(block.to_owned())
    .highlight_style(style_secondary())
    .select(0);

    f.render_widget(tabs, area);

    let title = "Projects";
    draw_project_block(
        f,
        chunks[1],
        ProjectTableProps {
            title: title.to_string(),
            inline_help: "| Containers <enter>".to_string(),
            project: &mut app.projects,
            table_headers: vec!["Name", "Path", "Size"],
            column_widths: vec![
                Constraint::Percentage(25),
                Constraint::Percentage(35),
                Constraint::Percentage(10),
            ],
        },
        |project| {
            let style = get_resource_row_style(project.is_deleted);
            Row::new(vec![
                Cell::from(project.name.to_owned()),
                Cell::from(
                    project
                        .path
                        .to_owned()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                ),
                Cell::from(human_bytes(project.size as f64)),
            ])
            .style(style)
        },
        app.light_theme,
        app.is_loading,
    );

    // f.render_widget(block, area);
}

fn get_resource_row_style(deleted: bool) -> Style {
    if deleted {
        style_success()
    } else {
        style_primary()
    }
}
// Utility methods

struct ProjectTableProps<'a, T> {
    title: String,
    inline_help: String,
    project: &'a mut StatefulTable<T>,
    table_headers: Vec<&'a str>,
    column_widths: Vec<Constraint>,
}

/// Draw a projects candidates i overview tab
fn draw_project_block<'a, B, T, F>(
    f: &mut Frame<B>,
    area: Rect,
    table_props: ProjectTableProps<'a, T>,
    row_cell_mapper: F,
    light_theme: bool,
    is_loading: bool,
) where
    B: Backend,
    F: Fn(&T) -> Row<'a>,
{
    let title = title_with_dual_style(table_props.title, table_props.inline_help, light_theme);
    let block = layout_block_top_border(title);
    if !table_props.project.items.is_empty() {
        let rows = table_props.project.items.iter().map(row_cell_mapper);

        let table = Table::new(rows)
            .header(table_header_style(table_props.table_headers, light_theme))
            .block(block)
            .highlight_style(style_highlight())
            .highlight_symbol(HIGHLIGHT)
            .widths(&table_props.column_widths);

        f.render_stateful_widget(table, area, &mut table_props.project.state);
    } else {
        loading(f, block, area, is_loading);
    }
}
