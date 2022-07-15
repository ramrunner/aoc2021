use crate::read_inp;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BitsPkt {
    result: u64, //this is first for ord and partialord to work using it.
    version: u32,
    type_id: u32,
    oper_sub_bits: u32,
    oper_sub_num: u32,
}

impl BitsPkt {
    fn new(v: u32, t: u32) -> Self {
        BitsPkt {
            result: 0,
            version: v,
            type_id: t,
            oper_sub_bits: 0,
            oper_sub_num: 0,
        }
    }

    fn set_result(&mut self, a: u64) {
        self.result = a;
    }

    fn is_literal(&self) -> bool {
        self.type_id == 4
    }

    fn set_oper_sub_bits(&mut self, a: u32) {
        self.oper_sub_bits = a;
    }

    fn set_oper_sub_num(&mut self, a: u32) {
        self.oper_sub_num = a;
    }
}

fn char_to_hexstr(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

// returns a result that can be an error made up by a heap alloc string
// https://users.rust-lang.org/t/help-understanding-return-for-box-dyn-error/33748
fn n_bits_to_dec(n: usize, s: &str) -> Result<u64, Box<dyn Error>> {
    if s.len() < n {
        return Err("not enough bits in string".into());
    }
    let mut ret = 0;
    for i in (0..n).rev() {
        //println!("doing {}", s.chars().rev().nth(i).unwrap());
        if s[..n].chars().rev().nth(i).unwrap() == '1' {
            ret += 2_u64.pow(i as u32)
        }
    }
    Ok(ret)
}

fn read_literal(is: &str) -> (u64, u32) {
    let mut s = is;
    let mut bitsparsed = 0;
    let mut done = false;
    let mut bitstr = Vec::<&str>::new();
    while !done && s.len() >= 5 {
        if s.chars().nth(0).unwrap() == '0' {
            done = true;
        }
        bitstr.push(&s[1..5]);
        s = &s[5..];
        bitsparsed += 5;
    }
    let finalstr = bitstr.join("");
    if let Ok(num) = n_bits_to_dec(finalstr.len(), &finalstr) {
        //println!("accumulated {} -> {}", finalstr, num);
        return (num, bitsparsed);
    }
    (0, 0)
}

// the lifetime + '_ was suggested by the compiler to show that it has the lifetime of acc
fn gen_getvers(acc: &mut u32) -> impl FnMut(&BitsPkt) + '_ {
    move |x: &BitsPkt| {
        *acc += x.version;
        //println!("current acc: {}", acc);
    }
}

//returns a packet and the offset to the next one
fn parse(pkt: &str, f: &mut impl FnMut(&BitsPkt)) -> (Option<BitsPkt>, u32) {
    let mut bitsparsed = 0;
    if let (Ok(ver), Ok(typ)) = (n_bits_to_dec(3, &pkt[0..3]), n_bits_to_dec(3, &pkt[3..6])) {
        bitsparsed += 6;
        let mut npkt = BitsPkt::new(ver as u32, typ as u32);
        if !npkt.is_literal() {
            if pkt.chars().nth(6).unwrap() == '0' {
                if let Ok(nbits) = n_bits_to_dec(15, &pkt[7..]) {
                    npkt.set_oper_sub_bits(nbits as u32);
                    bitsparsed += 16;
                }
            } else {
                if let Ok(np) = n_bits_to_dec(11, &pkt[7..]) {
                    npkt.set_oper_sub_num(np as u32);
                    bitsparsed += 12;
                }
            }
        } else {
            let (l, bp) = read_literal(&pkt[6..]);
            npkt.set_result(l);
            bitsparsed += bp;
        }
        //println!("new pkt: {:?} from {} and {}", npkt, ver, typ);
        f(&npkt);
        let mut rembits = npkt.oper_sub_bits;
        let mut rempkts = npkt.oper_sub_num;
        let mut opers = Vec::new();
        while rembits > 0 {
            let (npkt, ibits) = parse(&pkt[bitsparsed as usize..], f);
            rembits -= ibits;
            bitsparsed += ibits;
            opers.push(npkt.unwrap())
        }
        while rempkts > 0 {
            let (npkt, ibits) = parse(&pkt[bitsparsed as usize..], f);
            bitsparsed += ibits;
            rempkts -= 1;
            opers.push(npkt.unwrap())
        }
        match npkt.type_id {
            0 => {
                npkt.result = opers.iter().fold(0, |mut sum, val| {
                    sum += val.result;
                    sum
                })
            }

            1 => {
                npkt.result = opers.iter().fold(1, |mut prod, val| {
                    prod *= val.result;
                    prod
                })
            }

            2 => {
                npkt.result = opers.iter().min().unwrap().result;
            }

            3 => {
                npkt.result = opers.iter().max().unwrap().result;
            }

            5 => {
                npkt.result = if opers[0].result > opers[1].result {
                    1
                } else {
                    0
                };
            }

            6 => {
                npkt.result = if opers[0].result < opers[1].result {
                    1
                } else {
                    0
                };
            }

            7 => {
                npkt.result = if opers[0].result == opers[1].result {
                    1
                } else {
                    0
                };
            }

            4 => (),

            _ => {
                println!("wtf id is {} on {:?}", npkt.type_id, npkt)
            }
        };
        (Some(npkt), bitsparsed)
    } else {
        (None, 0)
    }
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(16, false);
    let mut acc = 0;
    let mut totres = 0;
    let mut lres;
    for l in inp.lines() {
        let bitstr = l.chars().map(|x| char_to_hexstr(x)).collect::<String>();
        println!("{}", l);
        let mut pf = gen_getvers(&mut acc);
        lres = parse(&bitstr, &mut pf);
        totres = lres.0.unwrap().result;
    }
    ans.push_str(&format!("[a] type sum of all packets {}\n", acc));
    ans.push_str(&format!("[b] result of outer packet {}\n", totres));
    ans
}
