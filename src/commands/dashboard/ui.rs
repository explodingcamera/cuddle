use super::Dashboard;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, dashboard: &mut Dashboard) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = dashboard
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(dashboard.title),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(dashboard.tabs.index);
    f.render_widget(tabs, chunks[0]);

    match dashboard.tabs.index {
        0 => draw_first_tab(f, dashboard, chunks[1]),
        // 1 => draw_second_tab(f, dashboard, chunks[1]),
        // 2 => draw_third_tab(f, dashboard, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
    draw_text(f, chunks[2]);
}
