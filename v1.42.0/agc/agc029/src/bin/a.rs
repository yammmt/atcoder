use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut vdq = VecDeque::new();
    for i in 0..s.len() {
        if s[i] == 'W' {
            if vdq.is_empty() {
                // really?
                continue;
            }

            let bidx = vdq.pop_front().unwrap();
            ans += i - bidx;
        }
        vdq.push_back(i);
    }
    println!("{}", ans);
}
