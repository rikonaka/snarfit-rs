use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

use crate::CHUNK_SIZE;
use crate::DEFAULT_CONFIG_FILE;
use crate::DEFAULT_OUTPUT_DIRECTORY;
use crate::FState;
use crate::Mode;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn set_config_file(path: &str) -> Result<PathBuf, StateError> {
    let path = fs::canonicalize(path)?;
    Ok(path)
}

fn set_output_directory(path: &str) -> Result<PathBuf, StateError> {
    let path = fs::canonicalize(path)?;
    Ok(path)
}

pub fn initialize_state() -> Result<FState, StateError> {
    let args: Vec<String> = env::args().collect();
    let s = FState {
        start_time: Local::now(),
        audit_file_open: false,
        audit_file_name: String::new(),
        audit_file: None,
        mode: Mode::None,
        input_file: None,
        fileswritten: 0,
        block_size: 512,
        config_file: set_config_file(DEFAULT_CONFIG_FILE)?,
        output_directory: set_output_directory(DEFAULT_OUTPUT_DIRECTORY)?,
        invocation: args.join(" "),
        chunk_size: CHUNK_SIZE,
        num_builtin: 0,
        skip: 0,
        time_stamp: Local::now(),
    };
    Ok(s)
}
