use crate::read_inp;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Change {
    Inc,
    Dec,
}

impl Point {
    fn from(&mut self, a: &str) {
        let pstrs = a.split(",").map(|x| x.trim());
        let nums: Vec<i32> = pstrs.map(|x| x.parse::<i32>().unwrap()).collect();
        self.x = nums[0];
        self.y = nums[1];
    }

    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn towards(&self, b: &Point) -> Vec<Point> {
        let large_diff = cmp::max(i32::abs(self.x - b.x), i32::abs(self.y - b.y));
        let small_diff = cmp::min(i32::abs(self.x - b.x), i32::abs(self.y - b.y));
        let (xdir, ydir) = match (self.x < b.x, self.y < b.y) {
            (true, true) => (Change::Inc, Change::Inc),
            (true, false) => (Change::Inc, Change::Dec),
            (false, true) => (Change::Dec, Change::Inc),
            (false, false) => (Change::Dec, Change::Dec),
        };

        let xlarger = i32::abs(self.x - b.x) >= i32::abs(self.y - b.y);
        // calculate the rate of small steps per large step
        let rate: f32 = small_diff as f32 / large_diff as f32;
        // calculate the inverse of that to modulo against it on the iter
        // here we match on small_diff which is int instead of f32, cause
        // fp matching is tricky
        let smallstep = match small_diff {
            0 => 0,
            _ => (1.0 / rate as f32) as i32,
        };
        let mut sx = self.x;
        let mut sy = self.y;
        let mut ret = Vec::<Point>::new();
        //register the starting point in the path
        ret.push(Point { x: sx, y: sy });
        for step in 0..large_diff {
            if xlarger {
                sx += match xdir {
                    Change::Inc => 1,
                    _ => -1,
                };
                if smallstep != 0 && step % smallstep == 0 {
                    sy += match ydir {
                        Change::Inc => 1,
                        _ => -1,
                    };
                }
            } else {
                sy += match ydir {
                    Change::Inc => 1,
                    _ => -1,
                };
                if smallstep != 0 && step % smallstep == 0 {
                    sx += match xdir {
                        Change::Inc => 1,
                        _ => -1,
                    };
                }
            }

            ret.push(Point { x: sx, y: sy });
        }
        ret
    }
}

#[derive(Debug)]
struct Board {
    dat: Vec<Vec<u32>>,
}

impl Board {
    fn new(x: usize, y: usize) -> Board {
        let mut ret = Board { dat: Vec::new() };
        for _x in 0..x + 1 {
            ret.dat.push(vec![0; y + 1 as usize]);
        }
        ret
    }

    fn draw_line(&mut self, a: Point, b: Point, only_straight: bool) {
        if only_straight && !(a.x == b.x || a.y == b.y) {
            return;
        }
        let path = a.towards(&b);
        for p in path.iter() {
            self.dat[p.y as usize][p.x as usize] += 1
        }
    }

    fn str(&self) -> String {
        let mut ret = "".to_owned();
        for i in self.dat.iter() {
            ret.push_str(&format!("{:?}\n", i).to_owned());
        }
        ret.to_string()
    }

    fn count_dangers(&self) -> u32 {
        let mut num = 0;
        for (_i, c) in self.dat.iter().enumerate() {
            for (j, _r) in c.iter().enumerate() {
                if c[j] > 1 {
                    num += 1;
                }
            }
        }
        num
    }
}

// makes a rectangle board of size a
fn make_board(a: usize) -> Board {
    let board = Board::new(a, a);
    board
}

pub fn run() -> String {
    let ansa: String;
    let ansb: String;
    let inp = read_inp(5, false);
    // do a first pass and accumulate all the dims of all points to figure out the board size
    let mut coordvec = Vec::<i32>::new();
    for (_i, e) in inp.lines().enumerate() {
        let pstrs: Vec<&str> = e.split("->").map(|x| x.trim()).collect();
        let mut p = Point::new();
        let mut p1 = Point::new();
        p.from(pstrs[0]);
        coordvec.push(p.x);
        coordvec.push(p.y);
        p1.from(pstrs[1]);
        coordvec.push(p1.x);
        coordvec.push(p1.y);
    }
    coordvec.sort();
    let dim = coordvec.last().unwrap();
    let mut brd = make_board(*dim as usize);
    //second pass to draw the lines
    for (_i, e) in inp.lines().enumerate() {
        let pstrs: Vec<&str> = e.split("->").map(|x| x.trim()).collect();
        let mut p = Point::new();
        let mut p1 = Point::new();
        p.from(pstrs[0]);
        p1.from(pstrs[1]);
        brd.draw_line(p, p1, true);
    }
    ansa = format!("[a] total dangers:{}", brd.count_dangers());

    //part b
    let mut brd1 = make_board(*dim as usize);
    for (_i, e) in inp.lines().enumerate() {
        let pstrs: Vec<&str> = e.split("->").map(|x| x.trim()).collect();
        let mut p = Point::new();
        let mut p1 = Point::new();
        p.from(pstrs[0]);
        p1.from(pstrs[1]);
        brd1.draw_line(p, p1, false);
    }

    ansb = format!("[b] total dangers:{}", brd1.count_dangers());

    [ansa, ansb].join("\n")
}
