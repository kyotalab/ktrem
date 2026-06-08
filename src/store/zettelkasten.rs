use crate::error::KtermError;
use crate::model::note::Zettel;
use std::path::Path;

// cards/ディレクトリ内のファイルを全て読み込む
pub fn load_all(dir: &Path) -> Result<Vec<Zettel>, KtermError> {
    todo!()
}

// 新規Zettelファイルを作成
pub fn create(dir: &Path, zettel: &Zettel) -> Result<(), KtermError> {
    todo!()
}

// Zettelファイルを削除
pub fn delete(dir: &Path, file_name: &str) -> Result<(), KtermError> {
    todo!()
}
