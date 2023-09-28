// WA: 察し しかしペナ率低い

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if n == 1 {
        println!(
            "{}",
            if s[0] == 'b' {
                "0"
            } else {
                "-1"
            }
        );
        return;
    }

    let mut dq = VecDeque::new();
    dq.push_back('b');
    let mut cur = 1;
    loop {
        match cur % 3 {
            0 => {
                dq.push_front('b');
                dq.push_back('b');
            }
            1 => {
                dq.push_front('a');
                dq.push_back('c');
            }
            2 => {
                dq.push_front('c');
                dq.push_back('a');
            }
            _ => unreachable!(),
        }

        if dq.len() == n && dq.iter().collect::<String>() == s.iter().collect::<String>() {
            println!("{}", cur);
            return;
        }

        if dq.len() >= n {
            println!("-1");
            return;
        }
        cur += 1;
    }
}
