use chrono::Local;
use std::fs::canonicalize;
use thiserror::Error;

use crate::FState;
use crate::Mode;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

fn set_config_file(path: &str) -> Result<> {
    let path = canonicalize(path)
}

pub fn initialize_state() {
    let s = FState {
        start_time: Local::now(),
        audit_file_open: false,
        mode: Mode::None,
        input_file: None,
        fileswritten: 0,
        block_size: 512,
    };
}
