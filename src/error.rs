use thiserror::Error;

#[derive(Error, Debug)]
pub enum KtermError {
    #[error("Unknown CardStatus: {0}")]
    UnknownCardStatus(String),
    #[error("Config read error: {0}")]
    ConfigReadError(String),
    #[error("Config parse error: {0}")]
    ConfigParseError(String),
    #[error("Config write error: {0}")]
    ConfigWriteError(String),
    #[error("Setup error: {0}")]
    SetupError(String),
}
