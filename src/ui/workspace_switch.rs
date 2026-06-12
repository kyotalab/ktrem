use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let state = match &app.workspace_switch_state {
        Some(state) => state,
        None => return,
    };

    let items: Vec<ListItem> = state
        .candidates
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let path_str = path.display().to_string();
            let style = if i == state.selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::styled(path_str, style))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select Workspace"),
    );

    frame.render_widget(list, area);
}
