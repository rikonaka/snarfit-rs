pub struct FileDB {
    pub suffix: String,
    pub header: Vec<u8>,
    pub footer: Option<Vec<u8>>,
    // marker are a method to search for any unique information besides just the header and the footer
    pub marker: Option<Vec<u8>>,
}

pub fn init_file_dbs() {
    let mut file_dbs = Vec::new();

    // JPG
    let fd = FileDB {
        suffix: String::from("jpg"),
        header: vec![0xff, 0xd8, 0xff],
        footer: Some(vec![0xff, 0xd9]),
        marker: None,
    };
    file_dbs.push(fd);

    // gif
    let fd = FileDB {
        suffix: String::from("gif"),
        header: vec![0x47, 0x49, 0x46, 0x38],
        footer: Some(vec![0x00, 0x3b]),
        marker: Some(vec![0x00, 0x00, 0x3b]),
    };
    file_dbs.push(fd);

    // bmp
    let fd = FileDB {
        suffix: String::from("bmp"),
        header: vec![b'B', b'M'],
        footer: None,
        marker: None,
    };
    file_dbs.push(fd);

    // wmv
    let fd = FileDB {
        suffix: String::from("wmv"),
        header: vec![0x30, 0x26, 0xb2, 0x75, 0x8e, 0x66, 0xcf, 0x11],
        footer: Some(vec![0xa1, 0xdc, 0xab, 0x8c, 0x47, 0xa9]),
        marker: None,
    };
    file_dbs.push(fd);

    // mov
    let fd = FileDB {
        suffix: String::from("mov"),
        header: vec![b'm', b'o', b'o', b'v'],
        footer: None,
        marker: None,
    };
    file_dbs.push(fd);

    // mp4
    let fd = FileDB {
        suffix: String::from("mov"),
        header: vec![0x00, 0x00, 0x00, 0x1c, 0x66, 0x74, 0x79, 0x70],
        footer: None,
        marker: None,
    };
    file_dbs.push(fd);

    // rif
    let fd = FileDB {
        suffix: String::from("rif"),
        header: vec![b'R', b'I', b'F', b'F'],
        footer: Some(vec![b'I', b'N', b'F', b'O']),
        marker: None,
    };
    file_dbs.push(fd);

    // htm
    let fd = FileDB {
        suffix: String::from("htm"),
        header: vec![b'<', b'h', b't', b'm', b'l'],
        footer: Some(vec![b'<', b'/', b'h', b't', b'm', b'l', b'>']),
        marker: None,
    };
    file_dbs.push(fd);

    // ole
    let fd = FileDB {
        suffix: String::from("htm"),
        header: vec![
            0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        footer: None,
        marker: None,
    };
    file_dbs.push(fd);
}
