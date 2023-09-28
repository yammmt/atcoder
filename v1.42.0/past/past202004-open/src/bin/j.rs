use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    let mut left_pos = VecDeque::new();
    for c in &s {
        match *c {
            '(' => {
                left_pos.push_back(ans.len());
                // ans.push(*c);
            }
            ')' => {
                let l = left_pos.pop_back().unwrap();
                let mut cur = vec![];
                for j in l..ans.len() {
                    cur.push(ans[j]);
                }
                for j in (0..cur.len()).rev() {
                    ans.push(cur[j]);
                }
            }
            _ => ans.push(*c),
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
