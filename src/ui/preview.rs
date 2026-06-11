use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, Tab};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // 選択中のノートのcontentを取得
    let content = match app.tab {
        Tab::Scratch => app
            .scratches
            .get(app.selected_index)
            .map(|s| s.content.as_str()),
        Tab::Zettelkasten => app
            .zettels
            .get(app.selected_index)
            .map(|z| z.content.as_str()),
    };

    match content {
        Some(c) if !c.is_empty() => render_content(frame, c, area),
        _ => render_empty(frame, area),
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

fn render_empty(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("No content")
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
