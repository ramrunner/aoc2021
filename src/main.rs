use aoc2021::make_days;
use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
    }
    let dnum: u8 = match args[1].parse::<u8>() {
        Ok(day) => day,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let days = make_days();
    println!("{}", days.solution[dnum as usize]());

    process::exit(0);
}

fn usage(name: &str) {
    println!("usage:{} day", name);
    process::exit(-1);
}
