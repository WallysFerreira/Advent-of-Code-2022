use std::fs::File;
use std::io::prelude::*;

pub fn get_contents(path: String) -> String {
    let mut f = File::open(path).expect("Could not open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    contents
}
