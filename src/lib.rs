use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
// i suspect that when i learn about macros
// this sillyness of adding the fn and mods will stop
mod d1;
mod d10;
mod d11;
mod d12;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

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
    ret.solution[5] = d5::run;
    ret.solution[6] = d6::run;
    ret.solution[7] = d7::run;
    ret.solution[8] = d8::run;
    ret.solution[9] = d9::run;
    ret.solution[10] = d10::run;
    ret.solution[11] = d11::run;
    ret.solution[12] = d12::run;
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
