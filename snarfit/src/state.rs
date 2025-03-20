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
use crate::SKIP;
use crate::SSpec;
use crate::Type;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
}

pub fn set_canonicalize_file(path: &str) -> Result<PathBuf, StateError> {
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
        input_file: String::new(),
        fileswritten: 0,
        block_size: 512,
        config_file: set_canonicalize_file(DEFAULT_CONFIG_FILE)?,
        output_directory: set_canonicalize_file(DEFAULT_OUTPUT_DIRECTORY)?,
        invocation: args.join(" "),
        chunk_size: CHUNK_SIZE,
        skip: SKIP,
    };
    Ok(s)
}

pub fn init_builtin(
    s: &mut FState,
    type_: Type,
    suffix: &str,
    header: &str,
    footer: &str,
    header_len: usize,
    footer_len: usize,
    max_len: usize,
    case_sen: usize,
) {
    let new_s_spec = SSpec {
        type_,
        suffix: suffix.to_string(),
        num_markers: 0,
        header_len,
        footer_len,
        max_len,
        found: 0,
        header: header.to_string(),
        footer: footer.to_string(),
        case_sen,
        comment: String::new(),
    };
}

pub fn set_search_def(file_type: &str) {
    let file_type_split: Vec<&str> = file_type.split(",").map(|x| x.trim()).collect();

    for f in file_type_split {
        if f == "jpg" || f == "jpeg" {
            init_builtin(s, type_);
        }
    }
}
