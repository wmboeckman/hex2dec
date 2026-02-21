pub mod cli;
// pub mod buff_reader;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// TODO: get this to work with the custom BufReader!

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}