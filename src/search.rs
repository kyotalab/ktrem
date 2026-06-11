use crate::model::note::{Scratch, Zettel};

// Scratchを検索（本文+タイトル）
pub fn search_scratch<'a>(scratches: &'a [Scratch], query: &str) -> Vec<&'a Scratch> {
    if query.is_empty() {
        return scratches.iter().collect();
    }

    let query_lower = query.to_lowercase();

    scratches
        .iter()
        .filter(|s| s.content.to_lowercase().contains(&query_lower))
        .collect()
}

// Zettelkastenを検索（本文+タイトル+タグ）
pub fn search_zettel<'a>(zettels: &'a [Zettel], query: &str) -> Vec<&'a Zettel> {
    if query.is_empty() {
        return zettels.iter().collect();
    }

    let query_lower = query.to_lowercase();

    zettels
        .iter()
        .filter(|z| {
            z.content.to_lowercase().contains(&query_lower)
                || z.tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&query_lower))
        })
        .collect()
}
