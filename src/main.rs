#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

use std::path::Path;
use std::fs;
use std::io::prelude::*;
pub mod ast;

#[cfg(test)]
mod tests;

fn read_file(filename: &str) -> String {
    let mut file = fs::File::open(Path::new(filename)).ok().unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).ok().unwrap();
    content
}


mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

fn main() {
    let input: String = read_file("./test.md");
    let source_lines = ast::get_source_lines(&input);

    match grammar::Document(&input, &source_lines) {
        Ok(r) => println!("Parsed as: {}", r),
        Err(e) => println!("Parse error: {:?}", e),
    }
}
