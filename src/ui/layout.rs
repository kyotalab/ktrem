use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{App, AppMode, Tab};
use crate::ui::{help, preview, scratch, wizard, workspace_switch, zettelkasten};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // 縦に分割（タブバー + メインエリア + ステータスバー）
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // タブバー
            Constraint::Min(0),    // メインエリア
            Constraint::Length(1), // ステータスバー
        ])
        .split(area);

    let tab_area = vertical[0];
    let main_area = vertical[1];
    let status_area = vertical[2];

    // タブバーを描画
    render_tabs(frame, app, tab_area);

    // ステータスバーを描画
    render_status(frame, app, status_area);

    // ウィザードモードの場合はウィザードを全画面で描画
    if app.mode == AppMode::Wizard {
        wizard::render(frame, app, main_area);
        return;
    }

    // ウィザードモードの場合はウィザードを全画面で描画
    if app.mode == AppMode::Wizard {
        wizard::render(frame, app, main_area);
        return;
    }

    // ワークスペース切り替えモードの場合は専用UIを全画面で描画
    if app.mode == AppMode::WorkspaceSwitch {
        workspace_switch::render(frame, app, main_area);
        return;
    }

    if app.mode == AppMode::Help {
        help::render(frame, main_area);
        return;
    }

    // メインエリアを横に分割（一覧40% + プレビュー60%）
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_area);

    let list_area = horizontal[0];
    let preview_area = horizontal[1];

    match app.tab {
        Tab::Scratch => scratch::render(frame, app, list_area),
        Tab::Zettelkasten => zettelkasten::render(frame, app, list_area),
    }

    preview::render(frame, app, preview_area);
}

fn render_tabs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let titles = vec![Line::from("Scratch"), Line::from("Zettelkasten")];

    let selected = match app.tab {
        Tab::Scratch => 0,
        Tab::Zettelkasten => 1,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("kterm"))
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_widget(tabs, area);
}

fn render_status(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (mode_text, mode_color) = match app.mode {
        AppMode::Normal => ("NORMAL", Color::Green),
        AppMode::Search => ("SEARCH", Color::Cyan),
        AppMode::Wizard => ("WIZARD", Color::Yellow),
        AppMode::TagEdit => ("TAG EDIT", Color::Magenta),
        AppMode::ConfirmDelete => ("DELETE", Color::Red),
        AppMode::WorkspaceSwitch => ("WORKSPACE", Color::Blue),
        AppMode::Help => ("HELP", Color::White),
    };

    let mut spans = vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default().fg(Color::Black).bg(mode_color),
        ),
        Span::raw("  "),
    ];

    // Searchモードの場合はクエリを表示
    if app.mode == AppMode::Search {
        let query = app.search_query.as_deref().unwrap_or("");
        spans.push(Span::styled("Search: ", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(format!("{}_", query)));
    }

    // TagEditモードの場合は入力中のタグを表示
    if app.mode == AppMode::TagEdit && let Some(state) = &app.tag_edit_state {
        spans.push(Span::styled("Tags: ", Style::default().fg(Color::Magenta)));
        spans.push(Span::raw(format!("{}_", state.input)));
    }

    if app.mode == AppMode::ConfirmDelete {
        spans.push(Span::styled(
            "Delete this note? [y/N]",
            Style::default().fg(Color::White),
        ));
    }

    let status = Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::DarkGray));

    frame.render_widget(status, area);
}
