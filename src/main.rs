use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let message = "Hello world!".as_bytes();

    
    writeTDB("test.tdb", message);
    readTDB("test.tdb");
}

fn writeTDB(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut fileWrite = File::create(path)?;
    fileWrite.write(data)?;

    Ok(())
}

fn readTDB(path: &str) -> std::io::Result<()> {
    let mut fileRead = File::open(path)?;

    let mut buf_reader = BufReader::new(fileRead);
    for lines in buf_reader.lines() {
        println!("{}", lines?);
    }


    Ok(())
}