use crate::read_inp;

fn do_neighbors(
    b: &mut [[u8; 12]; 12],
    i: usize,
    j: usize,
    f: fn(b: &mut [[u8; 12]; 12], i: usize, j: usize),
) {
    let mut ii = i - 1;
    let mut ij;
    while ii <= i + 1 {
        ij = j - 1;
        while ij <= j + 1 {
            if ii == i && ij == j {
                ij += 1;
                continue;
            }
            f(b, ii, ij);
            ij += 1;
        }
        ii += 1;
    }
}

fn incnot0(b: &mut [[u8; 12]; 12], i: usize, j: usize) {
    if b[i][j] != 0 {
        b[i][j] += 1;
    }
}

pub fn run() -> String {
    let mut ans: String = "".to_string();
    let mut board: [[u8; 12]; 12] = [[0; 12]; 12];
    let mut time = 0;
    let mut i;
    let mut j;
    let mut totflashes = 0;
    let mut thisroundflashed;
    let mut updates = true;
    let inp = read_inp(11, false);
    for (il, l) in inp.lines().enumerate() {
        l.chars().fold(1, |acc, x| {
            board[il + 1][acc] = x.to_digit(10).unwrap() as u8;
            acc + 1
        });
    }
    loop {
        thisroundflashed = 0;
        i = 1;
        while i < 11 {
            j = 1;
            while j < 11 {
                board[i][j] += 1;
                j += 1;
            }
            i += 1;
        }

        while updates {
            updates = false;
            i = 1;
            while i < 11 {
                j = 1;
                while j < 11 {
                    if board[i][j] > 9 {
                        updates = true;
                        board[i][j] = 0;
                        totflashes += 1;
                        thisroundflashed += 1;
                        do_neighbors(&mut board, i, j, incnot0);
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        // first star, is how many flashes at round 100 {
        if time == 99 {
            ans.push_str(&format!(
                "[a] at time 100 we have {} total octopus flashes\n",
                totflashes
            ));
        }

        updates = true;
        // second star, all octopuses flash together.
        if thisroundflashed == 100 {
            println!("sync flash at time {}", time);
            ans.push_str(&format!(
                "[b] sync flash of all octopuses at time {}\n",
                time + 1
            ));
            break;
        }

        time += 1;
    }

    ans.to_string()
}
