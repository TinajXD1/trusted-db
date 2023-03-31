mod disk_interaction;

fn main() {
    let message = "It doesn't work!".as_bytes();
    let path = String::from("databases/test");
    let column = String::from("disk_interaction");

    disk_interaction::create_tdb(&path);
    disk_interaction::create_column(&path, &column);
    disk_interaction::append_in_column(&path, &column, message);
    disk_interaction::read_column(&path, &column);
}
