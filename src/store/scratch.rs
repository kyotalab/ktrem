use std::path::PathBuf;
use crate::model::note::Scratch;
use crate::error::KtermError;

// scratch/ディレクトリ内のファイルを全て読み込む
pub fn load_all(dir: &PathBuf) -> Result<Vec<Scratch>, KtermError>;

// 新規Scratchファイルを作成（タイムスタンプでファイル名生成）
pub fn create(dir: &PathBuf) -> Result<Scratch, KtermError>;

// Scratchファイルを削除
pub fn delete(dir: &PathBuf, timestamp: &str) -> Result<(), KtermError>;
