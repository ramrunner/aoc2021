use crate::read_inp;

fn sum(x: i32) -> i32 {
    x * (x + 1) / 2
}

struct Targ {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

struct P {
    x: i32,
    y: i32,
}

fn in_targ(p: &P, tar: &Targ) -> bool {
    p.x >= tar.xmin && p.x <= tar.xmax && p.y <= tar.ymax && p.y >= tar.ymin
}

fn oob(p: &P, tar: &Targ) -> bool {
    p.x > tar.xmax || p.y < tar.ymin
}

#[derive(Debug, Copy, Clone)]
struct V {
    x: i32,
    y: i32,
}

fn sim(vs: V, tar: &Targ) -> (i32, bool) {
    let mut p = P { x: 0, y: 0 };
    let mut v = vs;
    let mut ymax = 0;
    let mut inside = false;
    loop {
        p.x += v.x;
        p.y += v.y;
        v.x = if v.x > 0 { v.x - 1 } else { 0 };
        v.y -= 1;
        if in_targ(&p, tar) {
            inside = true;
            break;
        }
        if oob(&p, tar) {
            break;
        }
        if p.y > ymax {
            ymax = p.y;
        }
    }
    (ymax, inside)
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(17, false);
    let commaind = inp.find(',').unwrap();
    let xstr = &inp[15..commaind];
    let ystr = &inp[commaind + 4..].trim();
    let xdim = xstr
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let ydim = ystr
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut xmin = 0;
    // we can estimate the xmin xmax since they are modelled by the sum
    // (every time we add a velocity that is one less than what we have)
    loop {
        if sum(xmin) >= xdim[0].abs().try_into().unwrap() {
            break;
        }
        xmin += 1;
    }
    let mut xmax = xmin;
    loop {
        if sum(xmax) >= (xdim[1]).abs().try_into().unwrap() {
            xmax += 1;
            break;
        }
        xmax += 1;
    }

    ans.push_str(&format!("[a] {}\n", sum(ydim[0])));
    let tar = Targ {
        xmin: xdim[0],
        xmax: xdim[1],
        ymin: ydim[0],
        ymax: ydim[1],
    };
    let mut yacc = 0;
    let mut vcounter = 0;
    for i in 0..=xdim[1] {
        for j in ydim[0]..(10 * xmax) {
            //the y limits are something large to bruteforce
            let (ytot, inside) = sim(
                V {
                    x: i as i32,
                    y: j as i32,
                },
                &tar,
            );
            if inside {
                vcounter += 1;
                if ytot > yacc {
                    yacc = ytot;
                }
            }
        }
    }
    ans.push_str(&format!("[b] {}\n", vcounter));
    ans
}
