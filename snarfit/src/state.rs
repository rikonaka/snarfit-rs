use chrono::Local;

use crate::FState;

pub fn initialize_state() {
    let s = FState {
        start_time: Local::now(),
        audit_file_open: false,
    };
}
