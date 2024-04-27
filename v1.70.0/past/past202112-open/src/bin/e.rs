use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let is_left = HashSet::from(['1', '2', '3', '4', '5']);
    let mut is_last_left = false;
    let mut ans = 0;
    for (i, &c) in n.iter().enumerate() {
        let is_now_left = is_left.contains(&c);
        if i == 0 {
            ans += 500;
        } else if i > 0 && c == n[i - 1] {
            ans += 301;
        } else if is_now_left == is_last_left {
            ans += 210
        } else {
            ans += 100;
        }
        is_last_left = is_now_left;
    }

    println!("{ans}");
}
