use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut stack = VecDeque::new();
    for c in s {
        match c {
            'C' => {
                if stack.len() < 2 {
                    stack.push_back('C');
                } else {
                    let b = stack.pop_back().unwrap();
                    let a = stack.pop_back().unwrap();
                    if a == 'A' && b == 'B' {
                        continue;
                    }
                    stack.push_back(a);
                    stack.push_back(b);
                    stack.push_back(c);
                }
            }
            // 入力が ABC に限定されているので C かそれ以外で判定してよい
            ab => stack.push_back(ab),
        }
    }

    println!("{}", stack.iter().collect::<String>());
}
