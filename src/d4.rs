use crate::read_inp;

#[derive(Debug)]
struct BingoBoard {
    data: [[u8; 5]; 5],
    set: [[bool; 5]; 5],
}

impl BingoBoard {
    fn from_string(&mut self, bs: &str) {
        for (il, l) in bs.lines().enumerate() {
            let cols = l.trim().split_whitespace();
            for (ic, c) in cols.enumerate() {
                self.data[il][ic] = c.parse::<u8>().unwrap();
            }
        }
    }
    // scans a board and if the number is found it is set to true
    fn set(&mut self, num: u8) {
        for (ir, r) in self.data.iter().enumerate() {
            for (ic, c) in r.iter().enumerate() {
                if *c == num {
                    self.set[ir][ic] = true;
                }
            }
        }
    }

    fn check(&self) -> bool {
        let mut tempc;
        let mut tempr;
        for i in 0..5 {
            tempc = true;
            tempr = true;
            for j in 0..5 {
                tempr = tempr && self.set[i][j];
                tempc = tempc && self.set[j][i];
            }
            if tempc || tempr {
                return true;
            }
        }
        return false;
    }

    fn score(&self, called: u8) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.set[i][j] {
                    sum += self.data[i][j] as u64;
                }
            }
        }
        sum *= called as u64;
        sum
    }
}

fn new_bingo_board() -> BingoBoard {
    BingoBoard {
        data: [[0; 5]; 5],
        set: [[false; 5]; 5],
    }
}

fn read_moves_boards() -> (Vec<BingoBoard>, String) {
    let fstr = read_inp(4, false);
    let lines = fstr.lines().collect::<Vec<&str>>();
    let moves = lines[0].clone().to_string();
    let mut boards = Vec::new();
    let mut sline = 2;
    while sline <= lines.len() - 5 {
        let mut bb = new_bingo_board();
        bb.from_string(&lines[sline..sline + 5].join("\n"));
        boards.push(bb);
        sline += 6
    }
    (boards, moves)
}

struct WinningBoard {
    bind: usize,
    score: u64,
    called: u8,
}

pub fn run() -> String {
    let (mut brds, mvs) = read_moves_boards();
    let mut won_id = Vec::new();
    let mut win_boards = Vec::new();
    let ansa: String;
    let ansb: String;
    for mstr in mvs.split(",") {
        let mnum = mstr.parse::<u8>().unwrap();
        for (ib, b) in brds.iter_mut().enumerate() {
            b.set(mnum);
            if b.check() && !won_id.contains(&ib) {
                won_id.push(ib);
                win_boards.push(WinningBoard {
                    bind: ib,
                    score: b.score(mnum),
                    called: mnum,
                });
            }
        }
    }
    let wb1 = win_boards.first().unwrap();
    let wb2 = win_boards.last().unwrap();
    ansa = format!(
        "[a] board {:?} wins first with score {} when {} was called",
        brds[wb1.bind], wb1.score, wb1.called
    );
    ansb = format!(
        "[b] board {:?} wins first with score {} when {} was called",
        brds[wb2.bind], wb2.score, wb2.called
    );
    [ansa, ansb].join("\n")
}
