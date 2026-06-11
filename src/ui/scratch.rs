use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::app::App;
use crate::model::note::Scratch;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let scratches = if let Some(query) = &app.search_query {
        crate::search::search_scratch(&app.scratches, query)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    } else {
        app.scratches.clone()
    };

    render_list(frame, &scratches, app.selected_index, area);
}

fn render_list(frame: &mut Frame, scratches: &[Scratch], selected_index: usize, area: Rect) {
    let items: Vec<ListItem> = scratches
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let line = render_item(s, i == selected_index);
            ListItem::new(Line::from(line))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Scratch"));

    let mut state = ListState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_item(scratch: &Scratch, _is_selected: bool) -> String {
    // タイムスタンプを "MM/DD HH:MM" 形式に変換
    // "202506061430" → "06/06 14:30"
    let timestamp = &scratch.timestamp;
    let formatted = if timestamp.len() == 12 {
        format!(
            "{}/{} {}:{}",
            &timestamp[4..6],   // MM
            &timestamp[6..8],   // DD
            &timestamp[8..10],  // HH
            &timestamp[10..12]  // MM
        )
    } else {
        timestamp.clone()
    };

    let preview = scratch.preview();

    // タイムスタンプ幅は固定（11文字）、残りをpreviewに使う
    format!("{}  {}", formatted, preview)
}
