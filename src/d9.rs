use crate::read_inp;
use std::collections::HashMap;

fn neighbor_indices(j: usize, i: usize, xsz: usize, ysz: usize) -> Vec<(usize, usize)> {
    let mut nvec = Vec::<(usize, usize)>::new();
    if j > 0 && j < ysz - 1 {
        nvec.push((j - 1, i));
        nvec.push((j + 1, i));
    }
    if i > 0 && i < xsz - 1 {
        nvec.push((j, i + 1));
        nvec.push((j, i - 1));
    }
    if j == 0 {
        nvec.push((j + 1, i));
    }
    if j == ysz - 1 {
        nvec.push((j - 1, i));
    }

    if i == 0 {
        nvec.push((j, i + 1));
    }
    if i == xsz - 1 {
        nvec.push((j, i - 1));
    }
    nvec
}

fn let_it_rain(
    board: &Vec<Vec<u32>>,
    lake: &mut HashMap<(usize, usize), bool>,
    p: (usize, usize),
    max: (usize, usize),
) {
    lake.insert(p, true);
    let nvec = neighbor_indices(p.0, p.1, max.0, max.1);
    for n in nvec {
        if board[n.0][n.1] >= board[p.0][p.1] && board[n.0][n.1] < 9 && !lake.contains_key(&n) {
            lake.entry(p).or_insert(true);
            // do dfs
            let_it_rain(board, lake, n, max);
        }
    }
}

pub fn run() -> String {
    let mut ans: String = "".to_string();
    let mut board = Vec::<Vec<u32>>::new();
    let mut minima = Vec::<(usize, usize)>::new();
    let inp = read_inp(9, false);
    for l in inp.lines() {
        let vline = l
            .chars()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        board.push(vline);
    }
    // total size of the board
    let ysz = board.len();
    let xsz = board[0].len();
    let mut minsum = 0;
    for j in 0..ysz {
        for i in 0..xsz {
            let nvec = neighbor_indices(j, i, xsz, ysz);
            let minimum = nvec.iter().fold(true, |acc, x| {
                if board[x.0][x.1] <= board[j][i] {
                    acc && false
                } else {
                    acc && true
                }
            });
            if minimum {
                minsum += board[j][i] + 1;
                minima.push((j, i));
            }
        }
    }
    ans.push_str(&format!("[a] sum of minimums {}\n", minsum));
    // for part 2 i will attempt to flood from the minima, and
    // make little lakes so i can go diving, instead of writing rust
    // at the airport. which seriously sucks.
    let mut lakesizes = Vec::<usize>::new();

    for p in minima {
        let mut my_lake = HashMap::<(usize, usize), bool>::new();
        let_it_rain(&board, &mut my_lake, p, (xsz, ysz));
        lakesizes.push(my_lake.len());
    }
    lakesizes.sort();
    lakesizes.reverse();
    let lakesizemul = lakesizes[..3].iter().fold(1, |acc, x| acc * x);
    ans.push_str(&format!(
        "[b] multiplying the size  of 3 biggest little lakes: {}\n",
        lakesizemul
    ));
    ans.to_string()
}
