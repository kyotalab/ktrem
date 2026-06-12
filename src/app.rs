use crate::config;
use crate::error::KtermError;
use crate::model::config::Config;
use crate::model::index::IndexJson;
use crate::model::note::{Scratch, Zettel};
use crate::store;
use std::collections::{HashMap, HashSet};

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
    pub backlinks_index: HashMap<String, Vec<String>>, // id -> このidをリンクしているZettelのidリスト
}

pub enum Tab {
    Scratch,
    Zettelkasten,
}

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Normal,        // 通常
    Search,        // 検索中
    Wizard,        // 昇格ウィザード
    TagEdit,       // タグ編集
    ConfirmDelete, // ノート削除
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

        let mut app = App {
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
            backlinks_index: HashMap::new(),
        };

        app.rebuild_backlinks_index();

        Ok(app)
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
        self.rebuild_backlinks_index();
        Ok(())
    }

    /// 上下移動
    pub fn move_up(&mut self) {
        match self.tab {
            Tab::Scratch => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Tab::Zettelkasten => {
                let visible = self.visible_zettel_indices();
                if let Some(pos) = visible.iter().position(|&i| i == self.selected_index)
                    && pos > 0
                {
                    self.selected_index = visible[pos - 1];
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.tab {
            Tab::Scratch => {
                if self.selected_index + 1 < self.scratches.len() {
                    self.selected_index += 1;
                }
            }
            Tab::Zettelkasten => {
                let visible = self.visible_zettel_indices();
                if let Some(pos) = visible.iter().position(|&i| i == self.selected_index)
                    && pos + 1 < visible.len()
                {
                    self.selected_index = visible[pos + 1];
                }
            }
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

    /// 折りたたみを考慮した、表示中のZettelのインデックス一覧
    fn visible_zettel_indices(&self) -> Vec<usize> {
        self.zettels
            .iter()
            .enumerate()
            .filter(|(_, zettel)| !self.is_zettel_hidden(zettel))
            .map(|(i, _)| i)
            .collect()
    }

    /// このZettelが折りたたみによって非表示かどうか
    pub fn is_zettel_hidden(&self, zettel: &Zettel) -> bool {
        self.zettels
            .iter()
            .any(|z| z.is_ancestor_of(&zettel.id) && !self.expanded_ids.contains(&z.id))
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

    pub fn open_confirm_delete(&mut self) {
        self.mode = AppMode::ConfirmDelete;
    }

    pub fn cancel_delete(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn confirm_delete(&mut self) -> Result<(), KtermError> {
        match self.tab {
            Tab::Scratch => {
                if let Some(scratch) = self.scratches.get(self.selected_index) {
                    let timestamp = scratch.timestamp.clone();
                    store::scratch::delete(&self.config.scratch_dir(), &timestamp)?;
                }
            }
            Tab::Zettelkasten => {
                if let Some(zettel) = self.zettels.get(self.selected_index) {
                    let file_name = zettel.file_name.clone();
                    let id = zettel.id.clone();
                    store::zettelkasten::delete(&self.config.cards_dir(), &file_name)?;
                    store::index::delete_entry(&mut self.index, &id)?;
                    store::index::save(&self.config.index_path(), &self.index)?;
                }
            }
        }

        self.reload()?;

        // 選択位置を調整
        let len = match self.tab {
            Tab::Scratch => self.scratches.len(),
            Tab::Zettelkasten => self.zettels.len(),
        };
        if self.selected_index >= len && len > 0 {
            self.selected_index = len - 1;
        }

        self.mode = AppMode::Normal;
        Ok(())
    }

    /// バックリンクインデックスを再構築
    fn rebuild_backlinks_index(&mut self) {
        let mut index: HashMap<String, Vec<String>> = HashMap::new();

        for zettel in &self.zettels {
            for linked_id in zettel.extract_links() {
                index.entry(linked_id).or_default().push(zettel.id.clone());
            }
        }

        self.backlinks_index = index;
    }

    /// 指定したIDのバックリンク一覧を取得
    pub fn backlinks(&self, id: &str) -> Vec<&Zettel> {
        self.backlinks_index
            .get(id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|linked_from| self.zettels.iter().find(|z| &z.id == linked_from))
                    .collect()
            })
            .unwrap_or_default()
    }
}
