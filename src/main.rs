use std::fs::File;
use std::io::Write;

//fn create_and_write_to_file() {
    //let mut file = File::create("my_files/example.txt").unwrap();
    //writeln!(file, "Hello, Rust file operations!").unwrap();
    //writeln!(file, "This is a new line.").unwrap();
//}

fn main() {
    let mut file = File::create("my_files/example.txt").unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
}