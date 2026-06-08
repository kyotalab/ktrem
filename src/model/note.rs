use chrono::{DateTime, Utc};

pub struct Zettel {
    pub id: String,              // "1/1/2"（表示ID）
    pub file_name: String,       // "1-1-2"（ファイル名、拡張子なし）
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub content: String,         // 本文全体
}

impl Zettel {
    pub fn title(&self) -> String {
        // contentからH1を抽出して返す
        todo!()
    }
}

pub struct Scratch {
    pub timestamp: String,       // "202506061430"
    pub content: String,         // 本文全体
}

impl Scratch {
    pub fn preview(&self) -> String {
        // H1があればH1、なければ本文冒頭を返す
        todo!()
    }
}
