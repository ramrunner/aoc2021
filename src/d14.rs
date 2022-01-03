use crate::read_inp;
use std::collections::HashMap;

fn pattern_to_pairs(a: &String) -> Vec<&str> {
    let mut ret = Vec::<&str>::new();
    for ind in 1..a.len() {
        ret.push(&a[ind - 1..ind + 1]);
    }
    ret
}

fn pair_match(p: &str, rules: &HashMap<String, String>) -> String {
    if let Some(ret) = rules.get(p) {
        ret.to_string()
    } else {
        "".to_string()
    }
}

fn merge_counts(a: &mut Frequencies, b: &Frequencies) {
    for (k, v) in &b.f {
        let times = a.f.entry(*k).or_insert(0);
        *times += v;
    }
}

fn iterate_seq(t: u32, pattern: String, rules: &HashMap<String, String>) -> Frequencies {
    let mut ret: Frequencies = Frequencies::new();
    let ptp = pattern_to_pairs(&pattern);
    let mut memo = HashMap::<(String, u32), Frequencies>::new();
    let mut first = true;
    for p in ptp {
        let r = recurse(0, t, p.to_string(), &rules, &mut memo, first);
        if first {
            first = false;
        }
        merge_counts(&mut ret, &r);
    }
    ret
}

fn recurse(
    t: u32,
    max: u32,
    pair: String,
    rules: &HashMap<String, String>,
    mem: &mut HashMap<(String, u32), Frequencies>,
    first: bool,
) -> Frequencies {
    let mut ret = Frequencies::new();
    if t <= max {
        //println!("{} [{}]/{} pair:{}", std::iter::repeat(" ").take(t as usize).collect::<String>(), t, dir, pair);
        if t == max {
            if first {
                // do both
                for p in pair.chars() {
                    ret.reg(p);
                }
            } else {
                // only do the second char
                ret.reg(pair.chars().nth(1).unwrap());
            }
        }
        let newmid = pair_match(&pair, &rules);
        let p1: String = [pair.chars().nth(0).unwrap(), newmid.chars().nth(0).unwrap()]
            .iter()
            .collect();
        let p1k: String = p1.clone();
        let p2: String = [newmid.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()]
            .iter()
            .collect();
        let p2k: String = p2.clone();
        if let Some(f1) = mem.get(&(p1k.clone(), t + 1)) {
            merge_counts(&mut ret, f1);
        } else {
            let res = recurse(t + 1, max, p1.clone(), &rules, mem, first);
            merge_counts(&mut ret, &res);
            mem.insert((p1k, t + 1), res);
        }

        if let Some(f2) = mem.get(&(p2k.clone(), t + 1)) {
            merge_counts(&mut ret, f2);
        } else {
            let res = recurse(t + 1, max, p2.clone(), &rules, mem, false);
            merge_counts(&mut ret, &res);
            mem.insert((p2k, t + 1), res);
        }
    }
    ret
}

#[derive(Debug)]
struct Frequencies {
    f: HashMap<char, u64>,
}

impl Frequencies {
    fn new() -> Frequencies {
        Frequencies {
            f: HashMap::<char, u64>::new(),
        }
    }

    fn reg(&mut self, a: char) {
        let times = self.f.entry(a).or_insert(0);
        *times += 1;
    }

    fn minmax(self) -> ((char, u64), (char, u64)) {
        let mut vec = vec![];
        for (k, v) in self.f {
            vec.push((k, v));
        }
        vec.sort_by(|a, b| a.1.cmp(&b.1));
        (vec[0], *vec.last().unwrap())
    }
}

pub fn run() -> String {
    let mut ret = "".to_string();
    let inp = read_inp(14, false);
    let mut pattern: String = "".to_string();
    let mut rules = HashMap::<String, String>::new();
    for (il, l) in inp.lines().enumerate() {
        match il {
            0 => {
                pattern = l.to_string();
            }
            1 => {}
            _ => {
                let parts = l.split(" -> ").collect::<Vec<_>>();
                rules.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
    let pattern1 = pattern.clone();
    let freqs = iterate_seq(10, pattern, &rules);
    let ans1 = freqs.minmax();
    ret.push_str(&format!(
        "[a] {:?} difference {}\n",
        ans1,
        ans1.1 .1 - ans1.0 .1
    ));
    let freqs1 = iterate_seq(40, pattern1, &rules);
    let ans2 = freqs1.minmax();
    ret.push_str(&format!(
        "[b] {:?} difference {}\n",
        ans2,
        ans2.1 .1 - ans2.0 .1
    ));

    ret
}
