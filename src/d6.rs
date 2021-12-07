use crate::read_inp;

//in the fish world fish live in
//discrete times. there can only be 8
//buckets of fish times, and each one of them
//is associated with a fish counter that track
//how many fish are in that stage of their lives

fn init_world(a: &str, w: &mut [u64; 9]) {
    let str_fld = a.split(",").collect::<Vec<&str>>();
    for i in str_fld.iter() {
        let inum = i.trim().parse::<usize>().unwrap();
        w[inum] += 1;
    }
}

fn sim(w: &mut [u64; 9]) {
    let mut ind = 8;
    let mut temp = [0 as u64; 9];
    while ind > 0 {
        temp[ind - 1] = w[ind];
        ind -= 1;
    }
    temp[8] = w[0];
    temp[6] += w[0];
    ind = 9;
    while ind > 0 {
        w[ind - 1] = temp[ind - 1];
        ind -= 1;
    }
}

pub fn run() -> String {
    let ansa: String;
    let ansb: String;
    let mut world = [0 as u64; 9];
    let inp = read_inp(6, false);
    init_world(&inp, &mut world);
    let mut times = 80;
    while times > 0 {
        sim(&mut world);
        times -= 1;
    }
    let sum = world.iter().fold(0u64, |acc, x| acc + x);
    ansa = format!("[a] t-80 fish world:{:?} total fish:{}", world, sum);
    // simulate 256-80 more times for part b
    let mut times = 256 - 80;
    while times > 0 {
        sim(&mut world);
        times -= 1;
    }
    let sum = world.iter().fold(0u64, |acc, x| acc + x);
    ansb = format!("[b] t-256 fish world:{:?} total fish:{}", world, sum);
    [ansa, ansb].join("\n")
}
