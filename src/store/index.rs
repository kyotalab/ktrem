use std::path::PathBuf;
use crate::model::index::{IndexJson, IndexEntry};
use crate::error::KtermError;

pub fn load(path: &PathBuf) -> Result<IndexJson, KtermError> {
    todo!()
}

pub fn save(path: &PathBuf, index: &IndexJson) -> Result<(), KtermError> {
    todo!()
}

pub fn create_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    todo!()
}

pub fn read_entry(index: &IndexJson, id: &str) -> Result<&IndexEntry, KtermError> {
    todo!()
}

pub fn update_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    todo!()
}

pub fn delete_entry(index: &mut IndexJson, id: &str) -> Result<(), KtermError> {
    todo!()
}
