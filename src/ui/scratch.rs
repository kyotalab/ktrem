use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;
use crate::model::note::Scratch;

// Scratchタブ全体を描画
pub fn render(frame: &mut Frame, app: &App, area: Rect);

// Scratch一覧を描画
fn render_list(frame: &mut Frame, scratches: &[Scratch], selected_index: usize, area: Rect);

// 一覧の1行を描画（タイムスタンプ + preview）
fn render_item(scratch: &Scratch, is_selected: bool) -> String;
