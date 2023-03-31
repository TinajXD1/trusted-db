use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{prelude::*, BufReader};

pub fn create_tdb(path: &str) -> std::io::Result<()> {
    let builder = DirBuilder::new().recursive(true).create(path)?;
    assert!(std::fs::metadata(path).unwrap().is_dir());

    Ok(())
}

pub fn create_column(path: &str, column: &str) -> std::io::Result<()> {
    let path_to_column = String::from(path) + &String::from(column);
    let mut file_append = OpenOptions::new().write(true).append(false).open(path_to_column)?;
    file_append.write("Start".as_bytes())?;

    Ok(())
}

pub fn append_in_column(path: &str, column: &str, data: &[u8]) -> std::io::Result<()> {
    let path_to_column = String::from(path) + &String::from(column);

    let mut file_append = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path_to_column)?;
    file_append.write(data)?;

    Ok(())
}

pub fn read_column(path: &str, column: &str) -> std::io::Result<()> {
    let path_to_column = String::from(path) + &String::from(column);

    let buf_reader = BufReader::new(path_to_column.as_bytes());
    for lines in buf_reader.lines() {
        println!("{}", lines?);
    }

    Ok(())
}
