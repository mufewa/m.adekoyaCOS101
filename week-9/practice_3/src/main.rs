use std::fs;

fm main() {
    fs::remove_file("data.txt").expect("Could not remove file");
    println!("file is removed");
}