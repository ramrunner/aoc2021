use crate::read_inp;

fn num_of_positions(a: &[u32]) -> usize {
    (a.last().unwrap() - a.first().unwrap()) as usize
}

fn calc_fuel(pos: &[u32], num: usize, sum: bool) -> Vec<i32> {
    let mut tot_fuel = vec![0 as i32; num];
    for p in pos.iter() {
        for ifp in 0..num {
            if sum {
                let max = i32::abs(*p as i32 - ifp as i32);
                let tot = (max * (max + 1)) / 2;
                // gauss ftw
                tot_fuel[ifp] += tot;
            } else {
                tot_fuel[ifp] += i32::abs(*p as i32 - ifp as i32);
            }
        }
    }
    tot_fuel
}

// this wasn't really needed but it got implemented nontheless...
fn get_n_best(n: usize, f: &Vec<i32>) -> Vec<(i32, usize)> {
    let mut withpos = vec![(0, 0); f.len()];
    for i in 0..f.len() {
        withpos[i] = (f[i], i);
    }
    withpos.sort_by(|x, y| x.0.cmp(&y.0));
    withpos[..n].to_vec()
}

pub fn run() -> String {
    let ansa: String;
    let ansb: String;

    let inp = read_inp(7, false);
    let fields = inp.split(",").collect::<Vec<_>>();
    let mut ipos = fields
        .iter()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    ipos.sort();
    let allpos = num_of_positions(&ipos);
    let fuel_per_pos = calc_fuel(&ipos, allpos, false);
    ansa = format!(
        "[a] 3 best consumptions and positions: {:?}",
        get_n_best(3, &fuel_per_pos)
    );
    // part b
    let fuel_per_pos = calc_fuel(&ipos, allpos, true);
    ansb = format!(
        "[b] 3 best consumptions and positions: {:?}",
        get_n_best(3, &fuel_per_pos)
    );

    [ansa, ansb].join("\n")
}
