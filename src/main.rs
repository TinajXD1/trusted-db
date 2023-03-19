mod disk_interaction;

fn main() {
    let message = "It doesn't work!".as_bytes();
    let path = String::from("databases/test.tdb");

    
    disk_interaction::write_tdb(&path, message);
    disk_interaction::append_tdb(&path, message);
    disk_interaction::read_tdb(&path);
}