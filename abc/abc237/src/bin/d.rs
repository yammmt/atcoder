use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // 逆から両端キューじゃないの？
    let mut vdq = VecDeque::new();
    vdq.push_back(n);
    for (i, c) in s.iter().rev().enumerate() {
        // println!("{} {}", i, c);
        if c == &'L' {
            vdq.push_back(n - i - 1);
        } else {
            vdq.push_front(n - i - 1);
        }
    }

    let mut ans = vec![];
    while let Some(cur) = vdq.pop_front() {
        ans.push(cur);
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
