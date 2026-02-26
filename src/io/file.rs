use std::{
    fs::File,
    io::{BufReader, BufWriter, prelude::*},
    path::Path
};

use log::{debug, error};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e);
            std::process::exit(2);
        }
    };
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write_lines_to_file(filename: impl AsRef<Path>, data: Vec<String>) {
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e);
            std::process::exit(2);
        }
    };

    let mut buf = BufWriter::new(file);
    
    for line in data {
        match buf.write((line + "\n").as_bytes()) {
            Ok(n) => {
                debug!("wrote line of lenth {} to file.", n-1);
            },
            Err(e) => {
                error!("{}", e);
                std::process::exit(2);
            } 
        };
    }
}