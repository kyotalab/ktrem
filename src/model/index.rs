use crate::error::KtermError;
use ::std::str::FromStr;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
}

impl FromStr for CardStatus {
    type Err = KtermError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
