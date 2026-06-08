use crate::model::config::Config;
use crate::model::note::{Zettel, Scratch};
use crate::model::index::IndexJson;

pub struct App {
    pub config: Config,
    pub tab: Tab,                        // 現在のタブ
    pub zettels: Vec<Zettel>,
    pub scratches: Vec<Scratch>,
    pub index: IndexJson,
    pub selected_index: usize,           // 一覧の選択位置
    pub search_query: Option<String>,    // 検索クエリ（Noneは検索していない状態）
    pub mode: AppMode,                   // 現在のモード
    pub wizard_state: Option<WizardState>, // ウィザードの状態（Noneは未開時）
}

pub enum Tab {
    Scratch,
    Zettelkasten,
}

pub enum AppMode {
    Normal,      // 通常
    Search,      // 検索中
    Wizard,      // 昇格ウィザード
    TagEdit,     // タグ編集
}

pub struct WizardState {
    pub id: String,
    pub title: String,
    pub tags: String,
    pub focused_field: WizardField,
}

pub enum WizardField {
    Id,
    Title,
    Tags,
}
