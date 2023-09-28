// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 999;
    for i in 0..s.len() {
        if i + 2 == s.len() {
            break;
        }

        let cur = 100 * s[i].to_digit(10).unwrap() as i16
            + 10 * s[i + 1].to_digit(10).unwrap() as i16
            + s[i + 2].to_digit(10).unwrap() as i16;
        ans = ans.min((753 - cur).abs());
    }
    println!("{}", ans);
}
