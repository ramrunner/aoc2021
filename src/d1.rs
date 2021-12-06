use crate::read_inp;

pub fn run() -> String {
    let mut strvec = Vec::new();
    let ansa: String;
    let ansb: String;
    let inp = read_inp(1, false);

    for (_i, elem) in inp.lines().enumerate() {
        strvec.push(elem);
    }

    let ivec: Vec<i32> = strvec.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    // although we start from [1..] the iter().enumerate() starts from 0 so we
    // don't need to do x-ivec[i-1], they are already offset by 1.
    let diffvec: Vec<i32> = ivec[1..]
        .iter()
        .enumerate()
        .map(|(i, x)| x - ivec[i])
        .collect();

    let num = diffvec
        .iter()
        .fold(0, |acc, &x| if x > 0 { acc + 1 } else { acc });
    ansa = format!("[a] total positive change: {}", num);

    // part 2 : create a ivec that has the sum of the next 3 elems on each elem
    let i2vec: Vec<i32> = ivec
        .iter()
        .enumerate()
        .map(|(i, x)| match i {
            i if i <= ivec.len() - 3 => x + ivec[i + 1] + ivec[i + 2],
            i if i == ivec.len() - 2 => x + ivec[i + 1],
            i if i == ivec.len() - 1 => x + 0,
            _ => 0,
        })
        .collect();
    let diff2vec: Vec<i32> = i2vec[1..]
        .iter()
        .enumerate()
        .map(|(i, x)| x - i2vec[i])
        .collect();
    let num2 = diff2vec
        .iter()
        .fold(0, |acc, &x| if x > 0 { acc + 1 } else { acc });

    ansb = format!("[b] total positive change: {}", num2);
    return [ansa, ansb].join("\n");
}
