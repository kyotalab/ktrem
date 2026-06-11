use crate::error::KtermError;
use crate::model::index::{CardStatus, IndexEntry, IndexJson};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> Result<IndexJson, KtermError> {
    let content =
        fs::read_to_string(path).map_err(|e| KtermError::StoreReadError(e.to_string()))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| KtermError::StoreParseError(e.to_string()))?;

    let version = json["version"]
        .as_u64()
        .ok_or_else(|| KtermError::StoreParseError("Missing version".to_string()))?
        as u32;

    let mut cards = HashMap::new();
    if let Some(cards_map) = json["cards"].as_object() {
        for (id, entry) in cards_map {
            let status = entry["status"]
                .as_str()
                .ok_or_else(|| KtermError::StoreParseError("Missing status".to_string()))?;
            let status = status.parse::<CardStatus>()?;

            let tags = entry["tags"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect();

            let created = entry["created"]
                .as_str()
                .ok_or_else(|| KtermError::StoreParseError("Missing created".to_string()))?
                .parse()
                .map_err(|e: chrono::ParseError| KtermError::StoreParseError(e.to_string()))?;

            let updated = entry["updated"]
                .as_str()
                .ok_or_else(|| KtermError::StoreParseError("Missing updated".to_string()))?
                .parse()
                .map_err(|e: chrono::ParseError| KtermError::StoreParseError(e.to_string()))?;

            cards.insert(
                id.clone(),
                IndexEntry {
                    status,
                    tags,
                    created,
                    updated,
                },
            );
        }
    }

    Ok(IndexJson { version, cards })
}

pub fn save(path: &Path, index: &IndexJson) -> Result<(), KtermError> {
    let mut cards = serde_json::Map::new();
    for (id, entry) in &index.cards {
        let entry_json = serde_json::json!({
            "status": entry.status.to_str(),
            "tags": entry.tags,
            "created": entry.created.to_rfc3339(),
            "updated": entry.updated.to_rfc3339(),
        });
        cards.insert(id.clone(), entry_json);
    }

    let json = serde_json::json!({
        "version": index.version,
        "cards": cards,
    });

    let content = serde_json::to_string_pretty(&json)
        .map_err(|e| KtermError::StoreParseError(e.to_string()))?;
    fs::write(path, content).map_err(|e| KtermError::StoreWriteError(e.to_string()))?;
    Ok(())
}

pub fn create_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    if index.cards.contains_key(id) {
        return Err(KtermError::EntryAlreadyExists(id.to_string()));
    }
    index.cards.insert(id.to_string(), entry);
    Ok(())
}

pub fn read_entry<'a>(index: &'a IndexJson, id: &str) -> Result<&'a IndexEntry, KtermError> {
    index
        .cards
        .get(id)
        .ok_or_else(|| KtermError::EntryNotFound(id.to_string()))
}

pub fn update_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    if !index.cards.contains_key(id) {
        return Err(KtermError::EntryNotFound(id.to_string()));
    }
    index.cards.insert(id.to_string(), entry);
    Ok(())
}

pub fn delete_entry(index: &mut IndexJson, id: &str) -> Result<(), KtermError> {
    index
        .cards
        .remove(id)
        .ok_or_else(|| KtermError::EntryNotFound(id.to_string()))?;
    Ok(())
}
