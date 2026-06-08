use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::error::KtermError;

pub enum CardStatus {
    Draft,
    Permanent,
}

impl CardStatus {
    pub fn to_str(&self) -> String {
        match self {
            CardStatus::Draft => "draft".to_string(),
            CardStatus::Permanent => "permanent".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<CardStatus, KtermError> {
        match s {
            "draft" => Ok(CardStatus::Draft),
            "permanent" => Ok(CardStatus::Permanent),
            _ => Err(KtermError::UnknownCardStatus(s.to_string())),
        }
    }
}

pub struct IndexEntry {
    pub status: CardStatus,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

pub struct IndexJson {
    pub version: u32,
    pub cards: HashMap<String, IndexEntry>,
}
