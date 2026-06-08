use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;
use crate::model::note::Zettel;

// Zettelkastenタブ全体を描画
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    todo!()
}

// Zettel一覧をツリー形式で描画
fn render_tree(frame: &mut Frame, zettels: &[Zettel], selected_index: usize, area: Rect) {
    todo!()
}

// ツリーの1行を描画（折りたたみ記号 + インデント + ID + title）
fn render_item(zettel: &Zettel, depth: usize, is_expanded: bool, is_selected: bool) -> String {
    todo!()
}

// ファイル名を表示IDに変換（"1-1-2" → "1/1/2"）
pub fn file_name_to_id(file_name: &str) -> String {
    todo!()
}

// IDから階層の深さを計算
fn calc_depth(id: &str) -> usize {
    todo!()
}
