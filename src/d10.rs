use crate::read_inp;
static PAIRS: [char; 8] = ['(', ')', '{', '}', '[', ']', '<', '>'];

enum BraceType {
    Open,
    Close,
}

enum LineType {
    Incomplete,
    Corrupted,
    Correct,
}

fn ch_to_brace_type(c: char) -> BraceType {
    let mut btype = BraceType::Close;
    for (ind, v) in PAIRS.iter().enumerate() {
        if *v == c && ind % 2 == 0 {
            btype = BraceType::Open;
            break;
        }
        if *v == c && ind % 2 != 0 {
            btype = BraceType::Close;
            break;
        }
    }
    btype
}

fn ch_to_pair(c: char) -> char {
    let mut ret = 'x';
    for (ind, v) in PAIRS.iter().enumerate() {
        if *v == c && ind % 2 == 0 {
            ret = PAIRS[ind + 1];
        } else if *v == c && ind % 1 == 0 {
            ret = PAIRS[ind - 1];
        }
    }
    ret
}

fn score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incompletescore(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn stack2score(s: &Vec<char>) -> u64 {
    let mut j = s.len() - 1;
    let mut score = 0;
    while j > 0 {
        score = score * 5;
        score += incompletescore(s[j]);
        if j > 0 {
            // we are trying to avoid underflowing.
            j = j - 1;
        } else {
            break;
        }
    }
    score
}

// returns a score and the type of line associated with it.
fn linelint(l: &[char]) -> (u64, LineType) {
    let mut stack: Vec<char> = Vec::new();
    let mut i = 0;
    let llen = l.len();
    let mut lscore = 0;
    let mut lt = LineType::Correct;
    while stack.len() > 0 {
        if stack.len() > 0 && i == llen {
            //println!("incomplete with score:{}", stack2score(&stack));
            lscore = stack2score(&stack);
            lt = LineType::Incomplete;
            break;
        }
        if stack.len() == 0 && i == llen {
            //println!("complete!");
            lscore = 0;
            lt = LineType::Correct;
            break;
        }
        let pair = ch_to_pair(l[i]);
        match ch_to_brace_type(l[i]) {
            BraceType::Open => {
                // push l[i] into the stack
                stack.push(l[i]);
            }
            BraceType::Close => {
                // we need to pop from the stack and match
                if let Some(top) = stack.pop() {
                    if pair == top {
                        //println!("we matched {} with {}", top, l[i]);
                    } else {
                        //println!("corrupted!");
                        lscore = score(l[i]);
                        lt = LineType::Corrupted;
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    (lscore, lt)
}

pub fn run() -> String {
    let mut ans: String = "".to_string();
    let mut acc = 0;
    let mut incscores = Vec::new();
    let inp = read_inp(10, false);
    for (i, l) in inp.lines().enumerate() {
        let lchars = l.chars().collect::<Vec<char>>();
        println!(" at line {}:{} ", i, l);
        let (score, ltype) = linelint(&lchars);
        match ltype {
            LineType::Corrupted => {
                acc += score;
            }
            LineType::Incomplete => {
                incscores.push(score);
            }
            LineType::Correct => {
                println!("line parsed correctly");
            }
        }
    }
    incscores.sort();
    ans.push_str(&format!("corrupted lines score {}\n", acc.to_string()));
    ans.push_str(&format!(
        "incomplete lines score {}\n",
        incscores[(incscores.len() / 2)]
    ));
    ans.to_string()
}
