use std::path::PathBuf;
use crate::model::note::Zettel;
use crate::error::KtermError;

// cards/ディレクトリ内のファイルを全て読み込む
pub fn load_all(dir: &PathBuf) -> Result<Vec<Zettel>, KtermError>;

// 新規Zettelファイルを作成
pub fn create(dir: &PathBuf, zettel: &Zettel) -> Result<(), KtermError>;

// Zettelファイルを削除
pub fn delete(dir: &PathBuf, file_name: &str) -> Result<(), KtermError>;
