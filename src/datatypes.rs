use std::u8;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

#[derive(Serialize, Deserialize)]
pub struct Paste {
    pub id: String,
    pub content: String,
    pub paste_type: PasteType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum PasteType {
    Text = 0,
    Url,
}

impl From<u8> for PasteType {
    fn from(int: u8) -> Self {
        match int {
            0 => Self::Text,
            1 => Self::Url,
            _ => panic!("Invalid paste type!"),
        }
    }
}
