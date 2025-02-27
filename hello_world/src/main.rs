use std::fs::File;
use std::io::Write;

fn main(){
    File::create("my_files/example.txt").unwrap();
    println!("{:?}", file);
    File::create("my_files/example1.txt").unwrap();
    println!("{:?}", file);
    
    // writeln!(file, "Hello, Rust file operations!").unwrap();
    // writeln!(file, "This is a new line.").unwrap();


}