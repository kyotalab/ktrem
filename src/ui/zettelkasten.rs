use crate::app::App;
use crate::model::note::Zettel;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let zettels = if let Some(query) = &app.search_query {
        crate::search::search_zettel(&app.zettels, query)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    } else {
        app.zettels.clone()
    };

    render_tree(frame, app, &zettels, area);
}

fn render_tree(frame: &mut Frame, app: &App, zettels: &[Zettel], area: Rect) {
    let max_id_len = zettels.iter().map(|z| z.id.len()).max().unwrap_or(4);

    let mut visible_items: Vec<(usize, &Zettel)> = Vec::new();

    for (i, zettel) in zettels.iter().enumerate() {
        if is_hidden(zettel, zettels, app) {
            continue;
        }
        visible_items.push((i, zettel));
    }

    let items: Vec<ListItem> = visible_items
        .iter()
        .map(|(original_i, zettel)| {
            let depth = calc_depth(&zettel.id);
            let has_children = has_children(zettel, zettels);
            let is_expanded = app.expanded_ids.contains(&zettel.id);
            let is_selected = *original_i == app.selected_index;
            let line = render_item(zettel, depth, has_children, is_expanded, max_id_len);
            let style = if is_selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::styled(line, style))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Zettelkasten"));

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_item(
    zettel: &Zettel,
    depth: usize,
    has_children: bool,
    is_expanded: bool,
    max_id_len: usize,
) -> String {
    let indent = "  ".repeat(depth);
    let toggle = if has_children {
        if is_expanded {
            "▼ "
        } else {
            "▶ "
        }
    } else {
        "  "
    };
    let id_padded = format!("{:<width$}", zettel.id, width = max_id_len);
    let title = zettel.title();

    format!("{}{}{}  {}", indent, toggle, id_padded, title)
}

fn is_hidden(zettel: &Zettel, all: &[Zettel], app: &App) -> bool {
    // 全ての祖先IDを確認し、一つでも折りたたまれていたら非表示
    all.iter().any(|z| {
        z.id != zettel.id && zettel.id.starts_with(&z.id) && !app.expanded_ids.contains(&z.id)
    })
}

fn has_children(zettel: &Zettel, all: &[Zettel]) -> bool {
    all.iter()
        .any(|z| z.id != zettel.id && z.id.starts_with(&zettel.id))
}

pub fn file_name_to_id(file_name: &str) -> String {
    Zettel::id_from_file_name(file_name)
}

fn calc_depth(id: &str) -> usize {
    if id.is_empty() {
        return 0;
    }

    let mut depth = 0;
    let mut prev_was_digit = false;

    for c in id.chars() {
        match c {
            '/' => {
                depth += 1;
                prev_was_digit = false;
            }
            c if c.is_alphabetic() && prev_was_digit => {
                depth += 1;
                prev_was_digit = false;
            }
            c if c.is_numeric() => {
                prev_was_digit = true;
            }
            _ => {
                prev_was_digit = false;
            }
        }
    }

    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    // file_name_to_idのテスト
    #[test]
    fn test_file_name_to_id_numeric() {
        assert_eq!(file_name_to_id("1-1-2"), "1/1/2");
    }

    #[test]
    fn test_file_name_to_id_alpha() {
        assert_eq!(file_name_to_id("1a"), "1a");
    }

    #[test]
    fn test_file_name_to_id_alpha_child() {
        assert_eq!(file_name_to_id("1a-1"), "1a/1");
    }

    #[test]
    fn test_file_name_to_id_deep() {
        assert_eq!(file_name_to_id("1-1-2-1"), "1/1/2/1");
    }

    // calc_depthのテスト
    #[test]
    fn test_calc_depth_root() {
        assert_eq!(calc_depth("1"), 0);
    }

    #[test]
    fn test_calc_depth_child() {
        assert_eq!(calc_depth("1/1"), 1);
    }

    #[test]
    fn test_calc_depth_alpha() {
        assert_eq!(calc_depth("1a"), 1);
    }

    #[test]
    fn test_calc_depth_alpha_child() {
        assert_eq!(calc_depth("1a/1"), 2);
    }

    #[test]
    fn test_calc_depth_deep() {
        assert_eq!(calc_depth("1/1/2/1"), 3);
    }
}
