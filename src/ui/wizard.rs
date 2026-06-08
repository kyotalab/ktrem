use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::{App, WizardState};
use crate::model::note::Zettel;

// ウィザード全体を描画
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    todo!()
}

// ツリー部分を描画（上半分）
fn render_tree(frame: &mut Frame, zettels: &[Zettel], area: Rect) {
    todo!()
}

// 入力フォームを描画（下半分）
fn render_form(frame: &mut Frame, state: &WizardState, area: Rect) {
    todo!()
}
