use crate::read_inp;
use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    b: Vec<Vec<u32>>,
    dx: u32,
    dy: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn d(&self, a: &Coord) -> u32 {
        let dx = a.x - self.x;
        let dy = a.y - self.y;
        //dx.pow(2) + dy.pow(2)
        dx + dy // manhattan distance
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    c: Coord,
    fs: u32, // dist from start
    te: u32, // dist to end
    f: u32,  // fs+te
}

impl Node {
    fn new(c: Coord, fs: u32, te: u32) -> Node {
        Node {
            c: c,
            fs: fs,
            te: te,
            f: fs + te,
        }
    }
}

impl Board {
    fn at(&self, c: &Coord) -> u32 {
        self.b[c.y as usize][c.x as usize]
    }

    fn from(a: &str) -> Board {
        let mut ret = Board {
            b: Vec::<Vec<u32>>::new(),
            dx: 0,
            dy: 0,
        };
        for l in a.lines() {
            let lvec = l
                .chars()
                .map(|x| x.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            ret.b.push(lvec);
        }
        ret.dx = ret.b[0].len() as u32;
        ret.dy = ret.b.len() as u32;
        ret
    }

    fn big_from(a: &str) -> Board {
        let mut ret = Board {
            b: Vec::<Vec<u32>>::new(),
            dx: 0,
            dy: 0,
        };
        for l in a.lines() {
            let mut lvec = l
                .chars()
                .map(|x| x.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let lveccp = lvec.clone();
            for i in 1..5 {
                let mut lv1 = lveccp
                    .iter()
                    .map(|x| if *x + i >= 10 { *x + i - 9 } else { *x + i })
                    .collect::<Vec<_>>();
                lvec.append(&mut lv1);
            }
            ret.b.push(lvec);
        }
        let retcp = ret.b.clone();
        for i in 1..5 {
            let mut nl = retcp
                .iter()
                .map(|l| {
                    l.iter()
                        .map(|x| if *x + i > 9 { *x + i - 9 } else { *x + i })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            ret.b.append(&mut nl);
        }
        ret.dx = ret.b[0].len() as u32;
        ret.dy = ret.b.len() as u32;
        ret
    }

    fn neighs(&self, a: &Coord) -> Vec<Coord> {
        let mut ret = Vec::<Coord>::new();
        let lastx = self.dx - 1;
        let lasty = self.dy - 1;
        match a.x {
            0 => {
                ret.push(Coord { x: 1, y: a.y });
            }
            _ if a.x == lastx => ret.push(Coord {
                x: lastx - 1,
                y: a.y,
            }),
            _ => {
                ret.push(Coord { x: a.x - 1, y: a.y });
                ret.push(Coord { x: a.x + 1, y: a.y });
            }
        }
        match a.y {
            0 => ret.push(Coord { x: a.x, y: 1 }),
            _ if a.y == lasty => ret.push(Coord {
                x: a.x,
                y: lasty - 1,
            }),
            _ => {
                ret.push(Coord { x: a.x, y: a.y - 1 });
                ret.push(Coord { x: a.x, y: a.y + 1 });
            }
        }
        ret
    }
    // performs an A* search and returns the total cost
    // of the best path found.
    fn Astar(&self, from: Coord, to: Coord) -> u32 {
        let mut openset = Vec::<Node>::new();
        let mut cameFrom = HashMap::<Node, Node>::new();
        let mut neighscores = HashMap::<Coord, u32>::new();
        let mut closedset = Vec::<Node>::new();
        let mut totrisk = 0;
        // initial risk will be subtracted since we never enter that node
        let initialrisk = self.at(&from);
        let start = Node::new(from.clone(), 0, from.d(&to));
        openset.push(start);
        while openset.len() != 0 {
            //sort openset by fscore
            openset.sort_by(|a, b| a.f.cmp(&b.f));
            let current = openset[0].clone();
            if current.c == to {
                let mut cur = current;
                let mut path = Vec::new();
                totrisk += self.at(&to);
                path.push((to, self.at(&to)));
                loop {
                    if let Some(par) = cameFrom.get(&cur) {
                        totrisk += self.at(&par.c);
                        path.push((par.c, self.at(&par.c)));
                        if par.c == from {
                            break;
                        }
                        cur = *par;
                    } else {
                        println!("wtf");
                        break;
                    }
                }
                break;
            } else {
                let cn = self.neighs(&current.c);
                openset.remove(0);
                closedset.push(current.clone());

                for n in cn {
                    let newfs = current.fs + self.at(&n);
                    if let seenscore = neighscores.entry(n).or_insert(newfs) {
                        if newfs <= *seenscore {
                            let child = Node::new(n, newfs, n.d(&to));
                            cameFrom.insert(child, current);
                            if openset.iter().any(|v| *v == child) {
                                continue;
                            }

                            openset.push(child);
                        }
                    }
                }
            }
        }
        totrisk - initialrisk
    }
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(15, false);
    let b = Board::from(&inp);
    let mut line = 0;
    let start = Coord { x: 0, y: 0 };
    let end = Coord {
        x: b.dx - 1,
        y: b.dy - 1,
    };
    let bbig = Board::big_from(&inp);

    let startb = Coord { x: 0, y: 0 };
    let endb = Coord {
        x: bbig.dx - 1,
        y: bbig.dy - 1,
    };
    ans.push_str(&format!(
        "[a] small board total cost {}\n",
        b.Astar(start, end)
    ));
    ans.push_str(&format!(
        "[b] big board total cost {}\n",
        bbig.Astar(startb, endb)
    ));
    ans
}
