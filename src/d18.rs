use crate::read_inp;
use std::fmt;
use std::cmp::max;


// to define a tree we use a recursive data struct
// that requires Box<> to store data in the heap.
#[derive(Debug, PartialEq, Clone)]
enum BTree<T> {
    Leaf(T),
    Node(Box<BTree<T>>, Box<BTree<T>>),
}

impl<T> BTree<T> {
    fn new_node(l: BTree<T>, r: BTree<T>) -> BTree<T> {
        BTree::Node(Box::new(l), Box::new(r))
    }

    fn new_leaf(e: T) -> BTree<T> {
        BTree::Leaf(e)
    }
    
    // finds the max depth below a node
    fn depth(&self) -> u32 {
        match self {
            BTree::Node(l,r) => max(l.depth(), r.depth()) + 1,
            BTree::Leaf(_) => 1,
        }
    }
}


// implementing IntoIterator so that the tree
// works on a for loop and returns its iterator
impl<T> IntoIterator for BTree<T> {
    type Item = T;
    type IntoIter = BTreeIterator<T>;

    fn into_iter(self) -> BTreeIterator<T> {
        BTreeIterator::new(self)
    }
}

// the actual iterator, has a vec to 
// store everything to be iterated upon
// on the left while recursing as much as
// it can and doing the left node. This 
// walks the tree in order.
struct BTreeIterator<T> {
    rnodes: Vec<BTree<T>>,
    current: Option<T>,
}

impl<T> BTreeIterator<T> {
    fn new(node: BTree<T>) -> BTreeIterator<T> {
        let mut it = BTreeIterator {
            rnodes: vec![],
            current: None,
        };
        // traverse the left adding the right ones
        it.do_left(node);
        it
    }

    fn do_left(&mut self, mut nod: BTree<T>) {
        // add the right branch
        loop {
            match nod {
                BTree::Node(l,r) => {
                    self.rnodes.push(*r);
                    nod = *l;
                },
                BTree::Leaf(x) => {
                    self.current = Some(x);
                    break;
                },
            }
        }
    }
}

impl<T> Iterator for BTreeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let res = self.current.take();

        if let Some(node) = self.rnodes.pop() {
            self.do_left(node)
        }

        res
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ValAction {
    v: u32,
    a: Vec<Action>,
}

struct SnailNum1 {
    t: BTree<ValAction>
}

fn new_val_action(val: u32) -> ValAction {
    ValAction {
        v: val,
        a: Vec::new(),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Add(u32),
    Remove(u32),
    Split,
    Delete,
}


// can be either expr,expr  | expr, num | num, expr | num, num
fn parse_numstring1(a: &str) -> SnailNum1 {
    let mut ret: SnailNum1;
    ret = SnailNum1 {
        t: BTree::new_leaf(new_val_action(9)),
    };
    let commaind = find_comma(a);
    if commaind == 0 {
        return ret;
    }
    let first = a.chars().nth(1).unwrap();
    let aftercomma = a.chars().nth(commaind+1).unwrap();
    match (first, aftercomma) {
        ('0'..='9', '0'..='9') => {
            //println!("taking 1");
            let lval = first.to_digit(10).unwrap();
            let l = BTree::new_leaf(new_val_action(lval));
            let rval = aftercomma.to_digit(10).unwrap();
            let r = BTree::new_leaf(new_val_action(rval));
            ret.t = BTree::new_node(l,r);
        },
        ('[', '0'..='9') => {
            //println!("taking 2");
            let l = parse_numstring1(&a[1..commaind]);
            let rval = aftercomma.to_digit(10).unwrap();
            let r = BTree::new_leaf(new_val_action(rval));
            ret.t = BTree::new_node(l.t, r);
        },
        ('0'..='9', '[') => {
            //println!("taking 3");
            let lval = first.to_digit(10).unwrap();
            let l = BTree::new_leaf(new_val_action(lval));
            let r = parse_numstring1(&a[commaind+1..]);
            ret.t = BTree::new_node(l, r.t);
        },
        ('[','[') => {
            //println!("taking 4");
            let l = parse_numstring1(&a[1..commaind]);
            let r = parse_numstring1(&a[commaind+1..]);
            ret.t = BTree::new_node(l.t, r.t);
        },
        (_,_) => {
            println!("wtf {}, {}", first, aftercomma);
        }
    }
    ret
}

// finds the next comma by balancing the number of left and
// right parens
fn find_comma(a: &str) -> usize {
    let mut popen = 0;
    for (ind, c) in a.chars().enumerate() {
        match c {
            '[' => popen += 1,
            ']' => popen -= 1,
            ',' => {
                if popen == 1 {
                    return ind;
                }
            },
            _ => continue,
        }
    }
    0
}

pub fn run() -> String {
    let mut ans = "".to_string();
    let inp = read_inp(18, true);
    let inpt = inp.trim();
    let mut tree = parse_numstring(&inpt, 0, &inpt);
    println!("final tree:{:?}", tree.t);
    //explode(&mut tree, 2);
    //println!("{}", tree);    
    ans
}

#[cfg(test)]
mod tests {
    use crate::d18::BTree;
    use crate::d18::Action;
    use crate::d18::parse_numstring;
    use crate::d18::parse_numstring1;
    use crate::d18::new_val_action;

    fn testTreeStrings() -> (Vec<BTree<u32>>,Vec<&'static str>) {
            let t1 = BTree::new_node(BTree::new_leaf(1), BTree::new_leaf(1));
            let s1 = "[1,1]";
            let t2 = BTree::new_node(
                    BTree::new_node(BTree::new_leaf(1), BTree::new_leaf(2)),
                    BTree::new_node(BTree::new_leaf(3), BTree::new_leaf(4)));
            let s2 = "[[1,2],[3,4]]";
            let t3 = BTree::new_node(
                    BTree::new_leaf(1), 
                    BTree::new_node(BTree::new_leaf(2), BTree::new_leaf(3)));
            let s3 = "[1,[2,3]]";
            let t4 = BTree::new_node(
                    BTree::new_node(BTree::new_leaf(1), BTree::new_leaf(2)),
                    BTree::new_leaf(3)); 
            let s4 = "[[1,2],3]";
            let tvec = vec![t1,t2,t3,t4];
            let svec = vec![s1,s2,s3,s4];
            (tvec, svec)
    }

    fn builddepth(l:u32) -> BTree<u32> {
        let mut ind = l;
        let mut l = BTree::new_leaf(0);
        let mut r = BTree::new_leaf(1);
        let mut lc : BTree<u32>;
        let mut rc : BTree<u32>;
        let mut lc1 : BTree<u32>;
        let mut rc1 : BTree<u32>;
        while ind > 2 {
            lc = l.clone();
            rc = r.clone();
            lc1 = l.clone();
            rc1 = r.clone();
            l = BTree::new_node(lc, rc);
            r = BTree::new_node(lc1, rc1);
            ind -= 1;
        }
        BTree::new_node(l,r)
    }

    #[test]
    fn test_numstring() {
            let (tvec, svec) = testTreeStrings();
            let numvec = svec.iter().map(|x|{parse_numstring(x, 0, x).t}).collect::<Vec<_>>();

            assert_eq!(tvec,numvec);
     }

    #[test]
    fn depth() {
        println!("{:?} => {}", builddepth(3), builddepth(3).depth());
        assert_eq!(builddepth(3).depth(), 3);
        assert_eq!(builddepth(4).depth(), 4);
        assert_eq!(builddepth(2).depth(), 2);
    }

    #[test]
    fn val_actions() {
            let (tvec, svec) = testTreeStrings();
            let numactionvec = svec.iter().map(|x|{parse_numstring1(x).t}).collect::<Vec<_>>();
            println!("{:?}",numactionvec);
            // add some actions to the first tree "[1,1]"
            let mut ftree = BTree::new_node(
                BTree::new_node(BTree::new_leaf(new_val_action(1)), BTree::new_leaf(new_val_action(2))), 
                BTree::new_node(BTree::new_leaf(new_val_action(3)), BTree::new_leaf(new_val_action(4))));
            for mut i in ftree {
                println!("iter {:?}", i);
                i.a.push(Action::Delete);
            }

            for i in ftree {
                println!("after iter {:?}", i);
            }
    }

}

/*

struct SnailNum<'a> {
    t: BTree<u32>,
    pstr: &'a str,
    istart: usize,
    iend: usize,
}



// can be either expr,expr  | expr, num | num, expr | num, num
fn parse_numstring<'a>(a: &'a str, istart: usize, pstr: &'a str) -> SnailNum<'a> {
    let mut ret: SnailNum;
    ret = SnailNum {
        t: BTree::new_leaf(0),
        pstr: "aaaa",
        istart: 0,
        iend: 0,
    };
    ret.pstr = pstr;
    ret.istart = istart;
    ret.iend = istart+a.len();
    let commaind = find_comma(a);
    if commaind == 0 {
        return ret;
    }
    let first = a.chars().nth(1).unwrap();
    //println!("commaind is {}", commaind);
    let aftercomma = a.chars().nth(commaind+1).unwrap();
    //println!("a:{}, is:{}, in parent str:{}", a, istart, &pstr[istart..ret.iend]);
    match (first, aftercomma) {
        ('0'..='9', '0'..='9') => {
            //println!("taking 1");
            let lval = first.to_digit(10).unwrap();
            let l = BTree::new_leaf(lval);
            let rval = aftercomma.to_digit(10).unwrap();
            let r = BTree::new_leaf(rval);
            ret.t = BTree::new_node(l,r);
        },
        ('[', '0'..='9') => {
            //println!("taking 2");
            let l = parse_numstring(&a[1..commaind], istart+1, pstr);
            let rval = aftercomma.to_digit(10).unwrap();
            let r = BTree::new_leaf(rval);
            ret.t = BTree::new_node(l.t, r);
        },
        ('0'..='9', '[') => {
            //println!("taking 3");
            let lval = first.to_digit(10).unwrap();
            let l = BTree::new_leaf(lval);
            let r = parse_numstring(&a[commaind+1..], istart+commaind+1, pstr);
            ret.t = BTree::new_node(l, r.t);
        },
        ('[','[') => {
            //println!("taking 4");
            let l = parse_numstring(&a[1..commaind], istart+1, pstr);
            let r = parse_numstring(&a[commaind+1..], istart+commaind+1, pstr);
            ret.t = BTree::new_node(l.t, r.t);
        },
        (_,_) => {
            println!("wtf {}, {}", first, aftercomma);
        }
    }
    ret
}
*/
