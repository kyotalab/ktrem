use crate::error::KtermError;
use crate::model::index::{IndexEntry, IndexJson};
use std::path::Path;

pub fn load(path: &Path) -> Result<IndexJson, KtermError> {
    todo!()
}

pub fn save(path: &Path, index: &IndexJson) -> Result<(), KtermError> {
    todo!()
}

pub fn create_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    todo!()
}

pub fn read_entry<'a>(index: &'a IndexJson, id: &str) -> Result<&'a IndexEntry, KtermError> {
    todo!()
}

pub fn update_entry(index: &mut IndexJson, id: &str, entry: IndexEntry) -> Result<(), KtermError> {
    todo!()
}

pub fn delete_entry(index: &mut IndexJson, id: &str) -> Result<(), KtermError> {
    todo!()
}
