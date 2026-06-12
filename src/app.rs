use crate::config;
use crate::error::KtermError;
use crate::model::config::Config;
use crate::model::index::IndexJson;
use crate::model::note::{Scratch, Zettel};
use crate::store;
use std::collections::HashSet;

pub struct App {
    pub config: Config,
    pub tab: Tab, // 現在のタブ
    pub zettels: Vec<Zettel>,
    pub scratches: Vec<Scratch>,
    pub index: IndexJson,
    pub selected_index: usize,             // 一覧の選択位置
    pub search_query: Option<String>,      // 検索クエリ（Noneは検索していない状態）
    pub mode: AppMode,                     // 現在のモード
    pub wizard_state: Option<WizardState>, // ウィザードの状態（Noneは未開時）
    pub expanded_ids: HashSet<String>,     // 展開中のZettelのID
    pub tag_edit_state: Option<TagEditState>,
}

pub enum Tab {
    Scratch,
    Zettelkasten,
}

#[derive(PartialEq)]
pub enum AppMode {
    Normal,  // 通常
    Search,  // 検索中
    Wizard,  // 昇格ウィザード
    TagEdit, // タグ編集
}

pub struct WizardState {
    pub id: String,
    pub title: String,
    pub tags: String,
    pub focused_field: WizardField,
}

#[derive(PartialEq)]
pub enum WizardField {
    Id,
    Title,
    Tags,
}

pub struct TagEditState {
    pub input: String, // カンマ区切りで編集
}

impl App {
    /// 起動時の初期化
    pub fn new() -> Result<Self, KtermError> {
        let config = config::load_or_setup()?;

        // scratch/ディレクトリがなければ作成
        let scratch_dir = config.scratch_dir();
        if !scratch_dir.exists() {
            std::fs::create_dir_all(&scratch_dir)
                .map_err(|e| KtermError::StoreWriteError(e.to_string()))?;
        }

        let index = store::index::load(&config.index_path())?;
        let mut zettels = store::zettelkasten::load_all(&config.cards_dir())?;
        let scratches = store::scratch::load_all(&config.scratch_dir())?;

        // index.jsonのメタ情報をZettelに合成
        for zettel in &mut zettels {
            if let Ok(entry) = store::index::read_entry(&index, &zettel.id) {
                zettel.tags = entry.tags.clone();
                zettel.created = entry.created;
                zettel.updated = entry.updated;
            }
        }

        Ok(App {
            config,
            tab: Tab::Scratch,
            zettels,
            scratches,
            index,
            selected_index: 0,
            search_query: None,
            mode: AppMode::Normal,
            wizard_state: None,
            expanded_ids: HashSet::new(),
            tag_edit_state: None,
        })
    }

    /// ファイルを再読み込み
    pub fn reload(&mut self) -> Result<(), KtermError> {
        let mut zettels = store::zettelkasten::load_all(&self.config.cards_dir())?;
        let scratches = store::scratch::load_all(&self.config.scratch_dir())?;

        // index.jsonのメタ情報をZettelに合成
        for zettel in &mut zettels {
            if let Ok(entry) = store::index::read_entry(&self.index, &zettel.id) {
                zettel.tags = entry.tags.clone();
                zettel.created = entry.created;
                zettel.updated = entry.updated;
            }
        }

        self.zettels = zettels;
        self.scratches = scratches;
        Ok(())
    }

    /// 上下移動
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let len = match self.tab {
            Tab::Scratch => self.scratches.len(),
            Tab::Zettelkasten => self.zettels.len(),
        };
        if self.selected_index + 1 < len {
            self.selected_index += 1;
        }
    }

    /// タブ切り替え
    pub fn toggle_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Scratch => Tab::Zettelkasten,
            Tab::Zettelkasten => Tab::Scratch,
        };
        self.selected_index = 0;
        self.search_query = None;
    }

    /// 折りたたみ開閉
    pub fn toggle_expand(&mut self) {
        if let Some(zettel) = self.zettels.get(self.selected_index) {
            let id = zettel.id.clone();
            if self.expanded_ids.contains(&id) {
                self.expanded_ids.remove(&id);
            } else {
                self.expanded_ids.insert(id);
            }
        }
    }

    /// 検索クエリ更新
    pub fn update_search(&mut self, query: String) {
        self.search_query = if query.is_empty() { None } else { Some(query) };
        self.selected_index = 0;
    }

    /// 昇格ウィザードを開く
    pub fn open_wizard(&mut self) {
        self.wizard_state = Some(WizardState {
            id: String::new(),
            title: String::new(),
            tags: String::new(),
            focused_field: WizardField::Id,
        });
        self.mode = AppMode::Wizard;
    }

    /// 昇格ウィザードを閉じる
    pub fn close_wizard(&mut self) {
        self.wizard_state = None;
        self.mode = AppMode::Normal;
    }

    pub fn open_tag_edit(&mut self) {
        if let Some(zettel) = self.zettels.get(self.selected_index) {
            let input = zettel.tags.join(", ");
            self.tag_edit_state = Some(TagEditState { input });
            self.mode = AppMode::TagEdit;
        }
    }

    pub fn close_tag_edit(&mut self) {
        self.tag_edit_state = None;
        self.mode = AppMode::Normal;
    }
}
