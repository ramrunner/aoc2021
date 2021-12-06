use crate::read_inp;

//this day was terrible. pardon the mess.

pub fn run() -> String {
    let mut strvec = Vec::new();
    let inp = read_inp(3, false);
    for (_, e) in inp.lines().enumerate() {
        strvec.push(e);
    }
    let ansa: String;
    let ansb: String;

    let tot = binstr_to_counts(&strvec);
    let fin = sums_to_bits(tot, false);
    let nfin = neg(&fin);
    //let a1 = bit2num(&fin);
    let finstr = &fin.iter().collect::<String>();
    let nfinstr = &nfin.iter().collect::<String>();
    let a1 = isize::from_str_radix(finstr, 2).unwrap();
    let a2 = isize::from_str_radix(nfinstr, 2).unwrap();
    //let a2 = bit2num(&neg(&fin));
    ansa = format!(
        "[a] gamma rate :{}, epsilon rate is {} and mul is {}",
        a1,
        a2,
        a1 * a2
    );
    //let finchar = fin.iter().map(|x| char::from_digit(*x as u32,2).unwrap()).collect();
    //let nfinchar = neg(&fin).iter().map(|x| char::from_digit(*x as u32,2).unwrap()).collect();
    let o2 = filter_columns(&strvec, false);
    let co2 = filter_columns(&strvec, true);
    let o2v = isize::from_str_radix(&o2, 2).unwrap();
    let co2v = isize::from_str_radix(&co2, 2).unwrap();
    ansb = format!("[b] o2: {}  co2: {}", o2v, co2v);
    [ansa, ansb].join("\n")
}

fn filter_columns(dat: &Vec<&str>, do_co2: bool) -> String {
    let mut set: Vec<_> = dat.to_vec();
    let mut col = 0;
    while set.len() > 1 {
        //count how many have 1 on this col
        let ones = set.iter().filter(|l| l.as_bytes()[col] == b'1').count();
        let bit = match (do_co2, ones * 2 >= set.len()) {
            (false, true) | (true, false) => b'1',
            _ => b'0',
        };
        set = set
            .into_iter()
            .filter(|l| l.as_bytes()[col] == bit)
            .collect();
        col += 1;
    }
    let ret = &set[0];
    return ret.to_string();
}

fn binstr_to_counts(strvec: &Vec<&str>) -> Vec<i32> {
    let tot = strvec.iter().fold(vec![0; strvec[0].len()], |mut acc, x| {
        let chars: Vec<char> = x.chars().collect();
        for i in 0..acc.len() {
            match chars[i] {
                '1' => {
                    acc[i] += 1;
                }
                '0' => {
                    acc[i] -= 1;
                }
                _ => println!("wtf"),
            }
        }
        return acc;
    });
    tot
}

fn sums_to_bits(tot: Vec<i32>, do_co2: bool) -> Vec<char> {
    let mut fin = vec!['0'; tot.len()];
    if !do_co2 {
        for i in 0..fin.len() {
            if tot[i] < 0 {
                fin[i] = '0';
            } else if tot[i] == 0 {
                fin[i] = '1';
            } else {
                fin[i] = '1';
            }
        }
    } else {
        for i in 0..fin.len() {
            if tot[i] < 0 {
                fin[i] = '1';
            } else if tot[i] == 0 {
                fin[i] = '0';
            } else {
                fin[i] = '0';
            }
        }
    }
    fin
}

//negates a bitstring encoded as a character.
fn neg(x: &Vec<char>) -> Vec<char> {
    let mut ret = vec!['0'; x.len()];
    for i in 0..x.len() {
        if x[i] == '1' {
            ret[i] = '0';
        } else {
            ret[i] = '1';
        }
    }
    ret
}
