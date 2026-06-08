use thiserror::Error;

#[derive(Error, Debug)]
pub enum KtermError {
    #[error("Unknown CardStatus: {0}")]
    UnknownCardStatus(String),
    // 他のモジュールのエラーも追加していく
}
