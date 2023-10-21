use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        x: usize,
        s: Chars,
    }

    let mut dq = VecDeque::new();
    for c in s {
        match c {
            'U' => match dq.pop_back() {
                None => dq.push_back('U'),
                Some('U') => {
                    dq.push_back('U');
                    dq.push_back('U');
                }
                Some('L') => {}
                Some('R') => {}
                _ => unreachable!(),
            },
            'L' => dq.push_back('L'),
            'R' => dq.push_back('R'),
            _ => unreachable!(),
        }
    }

    let mut cur = x;
    while let Some(c) = dq.pop_front() {
        match c {
            'U' => cur /= 2,
            'L' => cur *= 2,
            'R' => cur = 2 * cur + 1,
            _ => unreachable!(),
        }
    }

    println!("{cur}");
}
