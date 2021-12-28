use crate::read_inp;
use std::cmp;
use std::iter;

fn largest_dims(a: &[(u32, u32)]) -> (u32, u32) {
    let mut xmax = 0;
    let mut ymax = 0;
    for (ia, ib) in a {
        xmax = cmp::max(xmax, *ia);
        ymax = cmp::max(ymax, *ib);
    }
    (xmax + 1, ymax + 1)
}

fn tag_points(points: &[(u32, u32)], bm: &mut [bool], lp: u32) {
    for (px, py) in points {
        let ind = ((*py) * lp + (*px)) as usize;
        bm[ind] = true;
    }
}

fn hfold(a: &[bool], at: usize, elem_per_row: usize) -> Vec<bool> {
    let mut ret = Vec::new();
    let l1 = &a[..((at - 1) * elem_per_row as usize)];
    let l2 = &a[((at + 1) * elem_per_row as usize)..];
    let l3v = l2
        .chunks(elem_per_row)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .concat();
    let l3 = l3v.as_slice();
    let (blist, slist) = if l1.len() >= l3.len() {
        (l1, l3)
    } else {
        (l3, l1)
    };
    let both = (slist.iter().chain(iter::repeat(&false))).zip(blist.iter());

    for (t1, t2) in both {
        ret.push(*t1 || *t2);
    }
    ret
}

fn vfold(a: &[bool], at: usize, elem_per_row: usize) -> Vec<bool> {
    let mut ret = Vec::new();
    let nlines = a.len() / elem_per_row;
    for il in 0..nlines {
        let ind1 = il * elem_per_row;
        let ind2 = il * elem_per_row + (at);
        let ind3 = ind2 + 1;
        let ind4 = (il + 1) * elem_per_row;
        let l1 = &a[ind1..ind2];
        let l2 = a[ind3..ind4]
            .to_vec()
            .into_iter()
            .rev()
            .collect::<Vec<bool>>();
        let l3 = l2.as_slice();

        let (blist, slist) = if l1.len() >= l3.len() {
            (l1, l3)
        } else {
            (l3, l1)
        };
        let both = (slist.iter().chain(iter::repeat(&false))).zip(blist.iter());
        for (t1, t2) in both {
            ret.push(*t1 || *t2);
        }
    }
    ret
}

fn count_pixels(a: &[bool]) -> u32 {
    a.iter().fold(0, |acc, x| {
        if *x {
            return acc + 1;
        }
        acc
    })
}

fn print(a: &[bool], epr: u32) {
    let llen = a.len();
    for lines in 0..(llen / epr as usize) {
        let i1 = lines * epr as usize;
        let i2 = (lines + 1) * epr as usize;
        let l = &a[i1..i2];
        let cl = l
            .iter()
            .map(|x| if *x { '#' } else { '.' })
            .collect::<Vec<char>>();
        println!("{}", cl.into_iter().collect::<String>());
    }
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(13, false);
    let mut intpoints = Vec::<(u32, u32)>::new();
    let mut parse_points_done = false;
    let mut lp: (u32, u32) = (0, 0);
    let mut bmap: Vec<bool> = vec![];
    // elements per row. they change on vertical folds
    let mut epr = lp.0;
    let mut parta = true;
    let mut firstnum = 0;

    for l in inp.lines() {
        if l == "" {
            // once we read all the points , before the folds
            parse_points_done = true;
            lp = largest_dims(&intpoints);
            bmap = vec![false; ((lp.0) * (lp.1)).try_into().unwrap()];
            tag_points(&intpoints, &mut bmap, lp.0);
            epr = lp.0;
            continue;
        }
        if !parse_points_done {
            // reading points
            let p = l.split(",").collect::<Vec<&str>>();
            let pint = p
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            intpoints.push((pint[0], pint[1]));
        } else {
            let p = l.split("=").collect::<Vec<&str>>();
            let num = p[1].parse::<u32>().unwrap();
            match p[0].chars().rev().nth(0).unwrap() {
                'y' => {
                    println!("horizontal fold around {}!", num);
                    bmap = hfold(&bmap, num.try_into().unwrap(), epr.try_into().unwrap());
                }
                'x' => {
                    println!("vertical fold around {}!", num);
                    bmap = vfold(&bmap, num.try_into().unwrap(), epr.try_into().unwrap());
                    epr = cmp::max((epr - num - 1) as u32, (num - 1) as u32);
                }
                _ => println!("err"),
            }
            if parta {
                firstnum = count_pixels(&bmap);
                parta = false;
            }
        }
    }
    ans.push_str(&format!(
        "[a] total pixels after first rotation {}\n",
        firstnum
    ));
    print(&bmap, epr);
    ans
}
