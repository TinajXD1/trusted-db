use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

pub fn write_tdb(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file_append = OpenOptions::new().write(true).append(false).open(path)?;
    file_append.write(data)?;

    Ok(())
}

pub fn append_tdb(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file_append = OpenOptions::new().write(true).append(true).open(path)?;
    file_append.write(data)?;

    Ok(())
}

pub fn read_tdb(path: &str) -> std::io::Result<()> {
    let file_read = File::open(path)?;

    let buf_reader = BufReader::new(file_read);
    for lines in buf_reader.lines() {
        println!("{}", lines?);
    }

    Ok(())
}
