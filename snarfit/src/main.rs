use chrono::DateTime;
use chrono::Local;
use std::fs::File;

mod state;

pub struct FState {
    // f_state in foremost code
    pub mode: Mode,
    pub config_file: String,
    pub input_file: Option<String>,
    pub output_directory: String,
    pub start_time: DateTime<Local>,
    pub invocation: String,
    pub audit_file_name: String,
    pub audit_file: File,
    pub audit_file_open: bool,
    pub num_builtin: i32,
    pub chunk_size: i32,
    pub fileswritten: i32,
    pub block_size: i32,
    pub skip: i32,
    pub time_stamp: u64,
}

pub enum Mode {
    None,
    Verbose,
    Quiet,
    IndBlk,
    Quick,
    WriteAll,
    WriteAudit,
    MultiFile,
}

fn main() {
    println!("Hello, world!");
}
