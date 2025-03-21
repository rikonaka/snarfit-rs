use crate::Config;
use crate::SnarfitError;
use std::fs::File;
use std::os::unix::fs::FileExt;

use log::error;

pub fn process_disk(config: Config) -> Result<(), SnarfitError> {
    let mut file = match File::open(&config.input) {
        Ok(f) => f,
        Err(e) => {
            error!("open input disk failed: {}", e);
            return Err(e.into());
        }
    };

    let mut buffer = vec![0u8; config.chunk_size];

    file.read_exact_at(&mut buffer, 0)?;

    println!("{:02X?}", &buffer[..64]);
    Ok(())
}
