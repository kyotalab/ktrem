use crate::error::KtermError;
use crate::model::note::Zettel;
use chrono::Utc;
use std::fs;
use std::path::Path;

pub fn load_all(dir: &Path) -> Result<Vec<Zettel>, KtermError> {
    let mut zettels = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| KtermError::StoreReadError(e.to_string()))?;

    for entry in entries.flatten() {
        let path = entry.path();

        // .mdファイルのみ対象
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| KtermError::StoreReadError("Invalid file name".to_string()))?
            .to_string();

        let content =
            fs::read_to_string(&path).map_err(|e| KtermError::StoreReadError(e.to_string()))?;

        // id は file_name の "-" を "/" に変換
        let id = Zettel::id_from_file_name(&file_name);

        zettels.push(Zettel {
            id,
            file_name,
            tags: vec![],        // index.jsonと合成は呼び出し側で
            created: Utc::now(), // index.jsonと合成は呼び出し側で
            updated: Utc::now(), // index.jsonと合成は呼び出し側で
            content,
        });
    }

    // ID順にソート
    zettels.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(zettels)
}

pub fn create(dir: &Path, zettel: &Zettel) -> Result<(), KtermError> {
    let file_path = dir.join(format!("{}.md", zettel.file_name));

    if file_path.exists() {
        return Err(KtermError::EntryAlreadyExists(zettel.file_name.clone()));
    }

    fs::write(&file_path, &zettel.content)
        .map_err(|e| KtermError::StoreWriteError(e.to_string()))?;

    Ok(())
}

pub fn delete(dir: &Path, file_name: &str) -> Result<(), KtermError> {
    let file_path = dir.join(format!("{}.md", file_name));

    if !file_path.exists() {
        return Err(KtermError::EntryNotFound(file_name.to_string()));
    }

    fs::remove_file(&file_path).map_err(|e| KtermError::StoreWriteError(e.to_string()))?;

    Ok(())
}
