mod disk_interaction;

fn main() {
    let message = "Hello world!".as_bytes();

    
    disk_interaction::write_tdb("test.tdb", message);
    disk_interaction::append_tdb("test.db", message);
    disk_interaction::read_tdb("test.tdb");
}