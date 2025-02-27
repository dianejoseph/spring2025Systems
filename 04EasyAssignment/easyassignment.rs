use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    sid: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let sid = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { name, sid, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("name: {}", config.name);
    println!("SID: {}", config.sid);
    println!("port: {}", config.port);
}

fn main() {
    reading_from_file();
}