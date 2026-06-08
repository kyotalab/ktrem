use crate::model::note::{Scratch, Zettel};

// Scratchを検索（本文+タイトル）
pub fn search_scratch<'a>(scratches: &'a [Scratch], query: &str) -> Vec<&'a Scratch>;

// Zettelkastenを検索（本文+タイトル+タグ）
pub fn search_zettel<'a>(zettels: &'a [Zettel], query: &str) -> Vec<&'a Zettel>;
