use crate::read_inp;

pub fn run() -> String {
    let mut strvec = Vec::new();
    let inp = read_inp(2, false);
    let ansa: String;
    let ansb: String;

    for (_i, elem) in inp.lines().enumerate() {
        strvec.push(elem);
    }
    let tot = strvec.iter().fold((0, 0), |acc, x| {
        let split = x.split_whitespace();
        let fields: Vec<&str> = split.collect();
        match fields[0] {
            "forward" => (acc.0 + fields[1].parse::<i32>().unwrap(), acc.1),
            "up" => (acc.0, acc.1 - fields[1].parse::<i32>().unwrap()),
            "down" => (acc.0, acc.1 + fields[1].parse::<i32>().unwrap()),
            _ => (-1, -1),
        }
    });
    ansa = format!("[a] fw: {}  depth: {} mul: {}", tot.0, tot.1, tot.0 * tot.1);
    let tot = strvec.iter().fold((0, 0, 0), |acc, x| {
        let split = x.split_whitespace();
        let fields: Vec<&str> = split.collect();
        let f1 = fields[1].parse::<i32>().unwrap();
        match fields[0] {
            "forward" => (acc.0 + f1, acc.1 + (acc.2 * f1), acc.2),
            "up" => (acc.0, acc.1, acc.2 - f1),
            "down" => (acc.0, acc.1, acc.2 + f1),
            _ => (-1, -1, -1),
        }
    });
    ansb = format!("[b] fw: {}  depth: {} mul: {}", tot.0, tot.1, tot.0 * tot.1);
    [ansa, ansb].join("\n")
}
