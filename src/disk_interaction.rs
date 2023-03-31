use std::fs::{File, OpenOptions, DirBuilder};
use std::io::{prelude::*, BufReader};

pub fn create_tdb(path: &str) -> std::io::Result<()> {
  let mut db = path;
  
  let mut builder = DirBuilder::new().recursive(true).create(path)?;
  
  Ok(())
}

pub fn write_column(path: &str, column: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file_append = OpenOptions::new().write(true).append(false).open(path)?;
    file_append.write(data)?;

    Ok(())
}

pub fn append_in_column(path: &str, column: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file_append = OpenOptions::new().write(true).append(true).open(path)?;
    file_append.write(data)?;

    Ok(())
}

pub fn read_column(path: &str, column: &str) -> std::io::Result<()> {
    let file_read = File::open(path+column)?;

    let buf_reader = BufReader::new(file_read);
    for lines in buf_reader.lines() {
        println!("{}", lines?);
    }

    Ok(())
}
