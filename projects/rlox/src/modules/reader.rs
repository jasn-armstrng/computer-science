use std::fs;
use std::process;

pub fn read_source(filename: &String) {
    println!("SOURCE: {filename}");
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("ERROR: {err}!");
        process::exit(1);
    });

    println!("CONTENTS: {contents}");
}
