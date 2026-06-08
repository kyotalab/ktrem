use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;

// プレビューペイン全体を描画
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    todo!()
}

// Markdownテキストを描画（そのまま表示）
fn render_content(frame: &mut Frame, content: &str, area: Rect) {
    todo!()
}

// 何も選択されていない場合の空状態を描画
fn render_empty(frame: &mut Frame, area: Rect) {
    todo!()
}
