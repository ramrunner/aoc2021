use crate::read_inp;
use std::collections::HashMap;
//  aaa
//b     c
//  ddd
//e     f
//  ggg

fn charin(a: &str, b: char) -> bool {
    for v in a.chars() {
        if b == v {
            return true;
        }
    }
    false
}

fn charinarr(a: [char; 7], b: char) -> bool {
    for v in a.iter() {
        if b == *v {
            return true;
        }
    }
    false
}

fn freqmap(a: &Vec<&str>) -> HashMap<char, u32> {
    let mut ret = HashMap::new();
    for s in a {
        for c in s.chars() {
            let val = ret.entry(c).or_insert(0);
            *val += 1;
        }
    }
    ret
}

fn infer_mapping(sigs: &str) -> [char; 7] {
    let mut mapping = ['x'; 7];
    let mut strs = Vec::<_>::new();
    for s in sigs.split(" ") {
        if s != "" {
            strs.push(s.trim());
        }
    }
    strs.sort_by(|a, b| a.len().cmp(&b.len()));
    // now that they are sorted we know the first element represents 1 ,
    // the second 7, third is 4 and and the 10th is 8
    //println!("in infer sorted:{:?}", strs);
    let mix1 = strs[0];
    let mix7 = strs[1];
    let mix4 = strs[2];
    let mix8 = strs[9];
    // now we know the mapping for digit 1
    // we can also get a from being the top part of digit 7 not being in digit 1
    for c in mix7.chars() {
        if !charin(mix1, c) {
            mapping[0] = c;
        }
    }
    // now 6 has everything that 8 has but it doesn't have one that 1 has.
    // possible 6s are indexes 6, 7, 8 so by finding that we know 1
    let freqs = freqmap(&vec![strs[6], strs[7], strs[8]]);
    for (k, v) in &freqs {
        if *v == 2 && charin(mix1, *k) {
            mapping[2] = *k;
        }
    }
    for c in mix1.chars() {
        if c != mapping[2] {
            mapping[5] = c;
        }
    }
    let freqs = freqmap(&vec![strs[3], strs[4], strs[5]]);
    for (k, v) in &freqs {
        if *v == 1 && charin(mix4, *k) {
            mapping[1] = *k;
            break;
        }
    }

    // now we know a,b,c,f we can figure out d
    // in the digits 4 + the triple that is missing one (0, 6, 9)
    // b appears 4 times while d appears 3.
    // d is the one in digit 4 that is not in digit 1 or char b
    let notd = vec![
        mapping[1],
        mix1.as_bytes()[0] as char,
        mix1.as_bytes()[1] as char,
    ];
    'a: for c in mix4.chars() {
        for j in &notd {
            if c == *j {
                continue 'a;
            }
        }
        mapping[3] = c;
        break;
    }

    //compare 0 and 9 , now that we know d, we will infer that e is the other one with freq1
    let mut mix0 = "";
    let mut mix6 = "";
    let mut mix9 = "";
    for a in vec![strs[6], strs[7], strs[8]] {
        if !charin(a, mapping[3]) {
            mix0 = a;
        } else if !charin(a, mapping[2]) {
            mix6 = a;
        } else {
            mix9 = a;
        }
    }
    for a in mix0.chars() {
        if !charin(mix9, a) {
            mapping[4] = a;
            break;
        }
    }
    // now we will find mix3
    let mut mix3 = "";
    for s in vec![strs[3], strs[4], strs[5]] {
        if charin(s, mapping[0])
            && charin(s, mapping[2])
            && charin(s, mapping[3])
            && charin(s, mapping[5])
        {
            mix3 = s;
        }
    }
    let freqs = freqmap(&vec![mix3, mix1]);
    for (k, v) in &freqs {
        if *v == 1 && !charinarr(mapping, *k) {
            mapping[6] = *k;
        }
    }
    // using a digit that has g locate it since it doesn't exist still in
    // the mapping
    mapping
}

fn sigs_to_digit(mapping: [char; 7], sig: &str) -> u8 {
    let bmap = mapping
        .iter()
        .map(|x| charin(sig, *x))
        .collect::<Vec<bool>>();
    let mut tarr = [false; 7];
    for (iv, v) in bmap.iter().enumerate() {
        tarr[iv] = *v;
    }
    let dig = match tarr {
        // a    b     c    d     e     f    g
        [false, false, true, false, false, true, false] => 1,
        [true, false, true, true, true, false, true] => 2,
        [true, false, true, true, false, true, true] => 3,
        [false, true, true, true, false, true, false] => 4,
        [true, true, false, true, false, true, true] => 5,
        [true, true, false, true, true, true, true] => 6,
        [true, false, true, false, false, true, false] => 7,
        [true, true, true, true, true, true, true] => 8,
        [true, true, true, true, false, true, true] => 9,
        [true, true, true, false, true, true, true] => 0,
        _ => 100,
    };
    dig
}

pub fn run() -> String {
    let mut ans: String = "".to_string();
    let inp = read_inp(8, false);
    let lvec = inp.lines().collect::<Vec<&str>>();
    let mut acc0 = 0;
    for (il, l) in lvec.iter().enumerate() {
        let part = l.split("|").collect::<Vec<_>>();
        let acc = part[1].split(" ").fold(0, |acc, x| {
            if x.len() == 3 || x.len() == 4 || x.len() == 7 || x.len() == 2 {
                acc + 1
            } else {
                acc
            }
        });
        acc0 += acc;
    }
    ans.push_str(&format!(
        "[a] count of easy digits in the output {}\n",
        acc0
    ));

    let mut sum: u32 = 0;
    for (il, l) in lvec.iter().enumerate() {
        let part = l.split("|").collect::<Vec<_>>();
        let mp = infer_mapping(part[0].trim());
        let digstrs = part[1].trim().split(" ").collect::<Vec<_>>();
        let digs = digstrs
            .iter()
            .map(|x| sigs_to_digit(mp, x.trim()))
            .collect::<Vec<_>>();
        let mut num: u32 = 0;
        for i in 0..4 {
            num += digs[3 - i] as u32 * u32::pow(10, i as u32);
        }
        sum += num;
    }
    ans.push_str(&format!("[b] count of all decrypted digits {}\n", sum));
    ans.to_string()
}
