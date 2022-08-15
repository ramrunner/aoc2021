use crate::read_inp;

type SnailNum = Vec<(u8,u32)>;

fn parse_numstring(a: &str) -> SnailNum {
    let mut ret: SnailNum = Vec::new();
    let mut d : u8 = 0;
    for c in a.chars().collect::<Vec<_>>().iter() {
        match c {
            '[' => { d+=1; },
            ']' => { d-=1; },
            '0'..='9' => { 
                let num = c.to_digit(10).unwrap();
                ret.push((d, num));
            },
            _ => {},
         }
     }
    ret
}

fn add(a: SnailNum, b: SnailNum) -> SnailNum {
    let mut ret : SnailNum = a.to_vec();
    let mut other = b.to_vec();
    ret.append(&mut other);
    ret.iter_mut().for_each(|(d,_)| *d += 1);
    ret
}

fn splitat(a: &mut SnailNum, i: usize) {
    let (d,val) = a[i];
    let lval = val/2;
    let mut rval = lval;
    if val%2 == 1 {
        rval += 1
    }
    a[i] = (d+1, lval);
    a.insert(i+1, (d+1, rval))
}

fn explodeat(a: &mut SnailNum, i: usize) {
    if i > 0 {
        a[i-1].1 += a[i].1
    }
    if a.len() > i+2 {
        a[i+2].1 += a[i+1].1
    }
    // this will remove i and i+1 (vec changes)
    //println!("removing {:?}", a[i]);
    let (d,_) = a[i];
    //println!("before rem {}:{:?}", i, a);
    a.remove(i);
   // println!("after rem:{:?}", a);
    //println!("setting {:?} to 0", a[i]);
    a[i] = (d-1,0);
    //println!("after explode:{:?}", a);
}

fn reduce(a: &mut SnailNum) -> Option<&mut SnailNum> {
    let mut doexplode = false;
    let mut ati = 0;
    let mut dosplit = false;
    //let mut ret : SnailNum = Vec::new();
    // scan ltr for any node at depth 4 and explode it
    for (ind, elem) in a.iter().enumerate() {
        ati = ind;
        let (d,_) = elem;
        if d >= &5 {
            doexplode = true;
            break
        }
    }
    if !doexplode {
        for (ind, elem) in a.iter().enumerate() {
            ati = ind;
            let (_,val) = elem;
            if val > &9 {
                dosplit = true;
                break
            }
        }
    }
    if doexplode {
        //println!("explode {:?} at {}", a, ati);
        explodeat(a, ati);
    }

    if dosplit {
        //println!("split {:?} at {}", a, ati);
        splitat(a, ati);
    }

    if doexplode || dosplit {
        return Some(a)
    }
    None
}

fn getvals(a: &SnailNum) -> Vec<u32> {
    let mut ret : Vec<u32> = vec![];
    for (_, v) in a {
        ret.push(*v) 
    }
    ret
}

fn all_reduce(a: &mut SnailNum) {
    let mut cur = a;
    loop {
        if let Some(b) = reduce(cur) {
            cur = b
        } else {
            break
        }
    }
}

// because snailfish nums are vectors and should be cloned
// we implement this helper macro.
macro_rules! add {
    ($a:expr, $b:expr)=> {
        {
            add($a.to_vec(), $b.to_vec())
        }
    }
}


fn add_all_nums(a: &str) -> SnailNum {
    let mut cur : SnailNum = vec![];
    let lines = a.split("\n");
    for l in lines {
        let next = parse_numstring(&l);
        if cur.len() == 0 {
            cur = next;
        } else {
            //println!("{:?} + {:?}", cur, next);
            cur = add!(cur, next);
            //println!("pre reduce {:?}", getvals(&cur));
            all_reduce(&mut cur);
            //println!("apres reduce {:?}", getvals(&cur));
        }
    }
    cur
}

fn add_all_pairs(a: &str) -> (u32, usize, usize) {
    let lines = a.split("\n");
    let mut max : (u32, usize, usize) = (0,0,0);
    let mut alln : Vec<SnailNum> = vec![];
    for l in lines.clone() {
        let n1 = parse_numstring(&l);
        alln.push(n1)
    }
    let nlen = alln.len();
    for i in 0..nlen {
        for j in 0..nlen {
            if i == j {
                continue
            }
            let n1 = &alln[i];
            let n2 = &alln[j];
            let mut an12 = add!(n1,n2);
            let mut an21 = add!(n2,n1);
            all_reduce(&mut an12);
            all_reduce(&mut an21);
            let m1 = mag(&an12);
            let m2 = mag(&an21);
            if m1 > m2 {
                if m1 > max.0 {
                    max = (m1, i, j);
                } 
            } else {
                if m2 > max.0 {
                    max = (m2, j, i);
                }
            }
        }
    }
    max
}

fn mag(a: &SnailNum) -> u32 {
    let mut c = a.to_vec();
    fn recurse(a:&mut SnailNum, lvl: u8) -> u32 {
        let mut tot;
        if a.len() == 0 {
            return 0;
        }
        if a[0].0 == lvl { // we're the pair of someone who called it
            let val = a[0].1;
            a.remove(0);
            return val;
        }
        tot = 3*recurse(a, lvl+1);
        tot += 2*recurse(a, lvl+1);
        tot
    }
    recurse(&mut c,0)
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(18,false);
    let inpt = inp.trim();
    //let mut tree = parse_numstring(&inpt, 0, &inpt);
    let res = add_all_nums(inpt);
    let resval = getvals(&res);
    ans.push_str(& format!("[a] final num {:?}, vals {:?}, mag:{:?}\n", res, resval, mag(&res)));
    ans.push_str(& format!("[b] max {:?}\n", add_all_pairs(inpt)));
    //explode(&mut tree, 2);
    //println!("{}", tree);    
    ans
}

#[cfg(test)]
mod tests {
    use crate::d18::parse_numstring;
    use crate::d18::SnailNum;
    use crate::d18::add;
    use crate::d18::all_reduce;
    use crate::d18::getvals;
    use crate::d18::mag;

    fn create_numstrings() -> (Vec<SnailNum>, Vec<&'static str>) {
        let t1 : SnailNum = vec![(1,1),(1,1)];
        let s1 = "[1,1]";
        let t2 : SnailNum = vec![(2,1),(2,3), (3,2),(3,3)];
        let s2 = "[[1,3],[[2,3]]]";

        let tvec = vec![t1,t2];
        let svec = vec![s1,s2];
        (tvec,svec)
    }


    #[test]
    fn test_numstring() {
            let (tvec, svec) = create_numstrings();
            let numvec = svec.iter().map(|x|{parse_numstring(x)}).collect::<Vec<_>>();

            assert_eq!(tvec,numvec);
    }

    #[test]
    fn test_addnoreduce() {
            let (tvec, _) = create_numstrings();
            let newnum = add!(tvec[0], tvec[1]);
            println!("added {:?} and {:?} gave {:?}", tvec[0], tvec[1], newnum);
    }

    #[test]
    fn test_reduce() {
        let ns = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let ansvals = vec![0,7,4,7,8,6,0,8,1];
        let mut num = parse_numstring(ns);
        all_reduce(&mut num);
        let vals = getvals(&num);

        assert_eq!(vals, ansvals);
    }

    #[test]
    fn test_mag() {
        let nums = vec!["[9,1]","[1,9]","[[9,1],[1,9]]","[[1,2],[[3,4],5]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", "[[[[1,1],[2,2]],[3,3]],[4,4]]"];
        let pnums = nums.iter().map(|x| mag(&parse_numstring(x))).collect::<Vec<_>>();
        println!("mags {:?}", pnums);
    }

}
