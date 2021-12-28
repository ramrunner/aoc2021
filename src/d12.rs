use crate::read_inp;
use std::collections::HashMap;
use std::collections::VecDeque;

// static node id counter
static mut nid: u32 = 0;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    id: u32,
    is_tiny: bool,
}

#[derive(Debug)]
struct Graph {
    g: HashMap<u32, Vec<u32>>,
    nodes: Vec<Node>,
    totnodes: u32,
    names: Vec<String>,
}

#[derive(Debug)]
struct Step {
    c: u32,
    p: Vec<u32>,
    tv: bool,
}

impl Graph {
    // returns the index/id of the node
    fn register_node(&mut self, name: String) -> usize {
        let found = false;
        for (ind, n) in self.names.iter().enumerate() {
            if *n == name {
                return ind;
            }
        }
        let nn = Node {
            is_tiny: name.chars().all(|x| x.is_ascii_lowercase()),
            id: self.totnodes,
        };
        self.totnodes += 1;
        self.nodes.push(nn);
        self.names.push(name);
        self.nodes.len() - 1
    }

    fn node_ind_by_name(&self, name: &str) -> u32 {
        let mut ret = 0;
        for (ind, n) in self.names.iter().enumerate() {
            if *n == name {
                ret = ind;
            }
        }
        ret as u32
    }

    fn connect(&mut self, a: String, b: String) {
        let ia = self.register_node(a) as u32;
        let ib = self.register_node(b) as u32;

        if self.g.contains_key(&ia) {
            let mut links = self.g.get_mut(&ia).unwrap();
            links.push(ib);
            //println!("{:?} connects to {:?}",self.nodes[ia as usize], links);
        } else {
            self.g.insert(ia, vec![ib]);
        }
    }

    fn newGraph() -> Graph {
        Graph {
            g: HashMap::new(),
            nodes: Vec::new(),
            totnodes: 0,
            names: Vec::new(),
        }
    }

    fn bfs(&self, from: u32, to: u32, twice: bool) -> Vec<Vec<u32>> {
        let mut allpaths = Vec::new();
        self.bfs_helper(&from, to, &mut allpaths, twice);
        allpaths
    }

    fn bfs_helper(&self, from: &u32, to: u32, allpaths: &mut Vec<Vec<u32>>, dotwice: bool) {
        let starti = self.node_ind_by_name("start");
        let endi = self.node_ind_by_name("end");
        let mut count = 0;
        let first = Step {
            c: *from,
            p: vec![*from],
            tv: false, //twice visited starts as false
        };
        let mut queue = VecDeque::<Step>::new();
        queue.push_back(first);
        while queue.len() > 0 {
            let curr = queue.pop_front().unwrap();
            if curr.c == to {
                count += 1;
                allpaths.push(curr.p);
                continue;
            }
            let myg = &self.g;
            if let Some(links) = myg.get(&curr.c) {
                for l in links {
                    if !seen(*l, curr.p.clone()) {
                        let mut path = curr.p.clone();
                        if self.nodes[*l as usize].is_tiny {
                            path.push(*l);
                        }

                        let next = Step {
                            c: *l,
                            p: path,
                            tv: curr.tv,
                        };
                        queue.push_back(next);
                    } else if (*l != starti && *l != endi) && !curr.tv && dotwice {
                        let next = Step {
                            c: *l,
                            p: curr.p.clone(),
                            tv: true,
                        };
                        queue.push_back(next);
                    }
                }
            }
        }
    }
}

fn seen(a: u32, b: Vec<u32>) -> bool {
    for bi in b {
        if bi == a {
            return true;
        }
    }
    return false;
}

pub fn run() -> String {
    let mut ans: String = "".to_string();
    let inp = read_inp(12, false);
    let mut graph: Graph = Graph::newGraph();
    for (il, l) in inp.lines().enumerate() {
        let p = l.split("-").collect::<Vec<&str>>();
        graph.connect(p[0].to_string(), p[1].to_string());
        graph.connect(p[1].to_string(), p[0].to_string());
    }
    let start = graph.node_ind_by_name("start");
    let end = graph.node_ind_by_name("end");
    let allp1 = graph.bfs(start, end, false);
    let allp2 = graph.bfs(start, end, true);
    ans.push_str(&format!(
        "[a] visiting small caves once: {} total paths\n",
        allp1.len()
    ));
    ans.push_str(&format!(
        "[b] visiting one small cave twice: {} total paths\n",
        allp2.len()
    ));

    ans.to_string()
}
