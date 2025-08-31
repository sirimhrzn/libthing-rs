#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
use tsify::{declare, Tsify};

#[cfg_attr(target_family = "wasm", derive(tsify::Tsify, serde::Serialize, serde::Deserialize),tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(any(target_os = "android",target_os = "ios"), derive(uniffi::Error), uniffi(flat_error))]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Library not initialized")]
    LibraryUninitialized,

    #[cfg(target_family = "wasm")]
    #[error("{message}")]
    DateParseError {
        message: String,
    },

    #[cfg(not(target_family = "wasm"))]
    #[error("{0}")]
    DateParseError(#[from] jiff::Error),

    #[cfg(not(target_family = "wasm"))]
    #[error("{0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Failed to parse mapping.")]
    MappingCorrupted,
}

#[cfg(target_family = "wasm")]
impl From<jiff::Error> for AppError {
    fn from(err: jiff::Error) -> Self {
        AppError::DateParseError {
            message: err.to_string()
        }
    }
}
