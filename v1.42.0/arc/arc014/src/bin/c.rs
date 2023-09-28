// 10.5min

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut vdq = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        // println!("{:?}", vdq);
        if vdq.is_empty() {
            vdq.push_back(*c);
            continue;
        } else if vdq.len() == 1 {
            let front = vdq.pop_front().unwrap();
            if *c != front {
                vdq.push_front(front);
                vdq.push_back(*c);
            }
            continue;
        }

        let front = vdq.pop_front().unwrap();
        let back = vdq.pop_back().unwrap();
        if *c == front {
            vdq.push_back(back);
        } else if *c == back {
            vdq.push_front(front);
        } else if i == n - 1 || s[i + 1] == front {
            vdq.push_front(front);
            vdq.push_back(back);
            vdq.push_back(*c);
        } else {
            vdq.push_front(front);
            vdq.push_front(*c);
            vdq.push_back(back);
        }
    }
    // println!("{:?}", vdq);

    println!("{}", vdq.len());
}
