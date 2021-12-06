use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod d1;
mod d2;
mod d3;
mod d4;

pub struct Days {
    pub solution: [fn() -> String; 26],
}

pub fn make_days() -> Days {
    let mut ret = Days {
        solution: [notimpl; 26],
    };
    ret.solution[1] = d1::run;
    ret.solution[2] = d2::run;
    ret.solution[3] = d3::run;
    ret.solution[4] = d4::run;
    ret
}

pub fn notimpl() -> String {
    "not yet implemented".to_string()
}

pub fn read_file() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);
    let mut strvec = Vec::new();

    for (_i, elem) in reader.lines().enumerate() {
        let line = elem.unwrap();
        strvec.push(line);
    }
    strvec
}

pub fn read_file1() -> String {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let contents = fs::read_to_string(fname).expect("can't read file");
    contents
}

pub fn read_inp(day: u8, sample: bool) -> String {
    let fname = match sample {
        false => format!("data/d{}-inp", day),
        true => format!("data/d{}-sample", day),
    };
    let contents = fs::read_to_string(fname).expect("can't read file");
    contents
}
