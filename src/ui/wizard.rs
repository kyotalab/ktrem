use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::{App, WizardField, WizardState};
use crate::model::note::Zettel;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let wizard_state = match &app.wizard_state {
        Some(state) => state,
        None => return,
    };

    // 上下に分割（ツリー60% + フォーム40%）
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // ツリー
            Constraint::Percentage(40), // フォーム
        ])
        .split(area);

    render_tree(frame, &app.zettels, vertical[0]);
    render_form(frame, wizard_state, vertical[1]);
}

fn render_tree(frame: &mut Frame, zettels: &[Zettel], area: Rect) {
    let max_id_len = zettels.iter().map(|z| z.id.len()).max().unwrap_or(4);

    let items: Vec<ListItem> = zettels
        .iter()
        .map(|zettel| {
            let id_padded = format!("{:<width$}", zettel.id, width = max_id_len);
            let title = zettel.title();
            let line = format!("{}  {}", id_padded, title);
            ListItem::new(Line::from(line))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Zettelkasten - Select parent"),
    );

    frame.render_widget(list, area);
}

fn render_form(frame: &mut Frame, state: &WizardState, area: Rect) {
    // フォームを3行に分割（ID / Title / Tags）
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // ID
            Constraint::Length(3), // Title
            Constraint::Length(3), // Tags
        ])
        .split(area);

    let fields = [
        ("ID", &state.id, WizardField::Id, rows[0]),
        ("Title", &state.title, WizardField::Title, rows[1]),
        ("Tags", &state.tags, WizardField::Tags, rows[2]),
    ];

    for (label, value, field, rect) in &fields {
        let is_focused =
            std::mem::discriminant(&state.focused_field) == std::mem::discriminant(field);

        let style = if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(*label, style));

        let paragraph = Paragraph::new(value.as_str()).block(block);
        frame.render_widget(paragraph, *rect);
    }
}
