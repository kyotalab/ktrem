use crate::error::KtermError;
use crate::model::note::Scratch;
use std::path::Path;

// scratch/ディレクトリ内のファイルを全て読み込む
pub fn load_all(dir: &Path) -> Result<Vec<Scratch>, KtermError> {
    todo!()
}

// 新規Scratchファイルを作成（タイムスタンプでファイル名生成）
pub fn create(dir: &Path) -> Result<Scratch, KtermError> {
    todo!()
}

// Scratchファイルを削除
pub fn delete(dir: &Path, timestamp: &str) -> Result<(), KtermError> {
    todo!()
}
