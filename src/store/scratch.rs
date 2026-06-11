use std::fs;
use std::path::Path;

use chrono::Local;

use crate::error::KtermError;
use crate::model::note::Scratch;

pub fn load_all(dir: &Path) -> Result<Vec<Scratch>, KtermError> {
    let mut scratches = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| KtermError::StoreReadError(e.to_string()))?;

    for entry in entries.flatten() {
        let path = entry.path();

        // .mdファイルのみ対象
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let timestamp = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| KtermError::StoreReadError("Invalid file name".to_string()))?
            .to_string();

        let content =
            fs::read_to_string(&path).map_err(|e| KtermError::StoreReadError(e.to_string()))?;

        scratches.push(Scratch { timestamp, content });
    }

    // タイムスタンプ降順（新しい順）
    scratches.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(scratches)
}

pub fn create(dir: &Path) -> Result<Scratch, KtermError> {
    // タイムスタンプでファイル名生成
    let timestamp = Local::now().format("%Y%m%d%H%M").to_string();
    let file_path = dir.join(format!("{}.md", timestamp));

    // 空ファイルを作成
    fs::write(&file_path, "").map_err(|e| KtermError::StoreWriteError(e.to_string()))?;

    Ok(Scratch {
        timestamp,
        content: String::new(),
    })
}

pub fn delete(dir: &Path, timestamp: &str) -> Result<(), KtermError> {
    let file_path = dir.join(format!("{}.md", timestamp));

    if !file_path.exists() {
        return Err(KtermError::EntryNotFound(timestamp.to_string()));
    }

    fs::remove_file(&file_path).map_err(|e| KtermError::StoreWriteError(e.to_string()))?;

    Ok(())
}
