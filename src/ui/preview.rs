use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, Tab};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    match app.tab {
        Tab::Scratch => {
            let content = app
                .scratches
                .get(app.selected_index)
                .map(|s| s.content.as_str());
            match content {
                Some(c) if !c.is_empty() => render_content(frame, c, area),
                _ => render_empty(frame, area),
            }
        }
        Tab::Zettelkasten => {
            let zettel = app.zettels.get(app.selected_index);

            match zettel {
                Some(z) if !z.content.is_empty() => {
                    // 上下に分割（本文 + バックリンク）
                    let vertical = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(0),    // 本文
                            Constraint::Length(6), // バックリンク
                        ])
                        .split(area);

                    render_content(frame, &z.content, vertical[0]);
                    render_backlinks(frame, app, &z.id, vertical[1]);
                }
                _ => render_empty(frame, area),
            }
        }
    }
}

fn render_content(frame: &mut Frame, content: &str, area: Rect) {
    let lines: Vec<Line> = content
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_backlinks(frame: &mut Frame, app: &App, id: &str, area: Rect) {
    let backlinks = app.backlinks(id);

    let items: Vec<ListItem> = if backlinks.is_empty() {
        vec![ListItem::new(Line::styled(
            "No backlinks",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        backlinks
            .iter()
            .map(|z| {
                let line = format!("{}  {}", z.id, z.title());
                ListItem::new(Line::from(line))
            })
            .collect()
    };

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Backlinks"));

    frame.render_widget(list, area);
}

fn render_empty(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("No content")
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
