use std::fs::File;
use std::io::prelude::*;

pub fn list() -> Vec<String> {
    let mut file = File::open("./words.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let words: Vec<String> = contents
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    words
}
