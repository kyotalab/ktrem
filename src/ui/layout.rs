use ratatui::Frame;
use ratatui::layout::{Layout, Constraint, Direction};
use crate::app::App;

// 画面全体を描画
pub fn render(frame: &mut Frame, app: &App) {
    // 縦に分割（タブバー + メインエリア）
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // タブバー
            Constraint::Min(0),     // メインエリア
        ]);

    // メインエリアを横に分割（一覧40% + プレビュー60%）
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),  // 一覧
            Constraint::Percentage(60),  // プレビュー
        ]);
}
