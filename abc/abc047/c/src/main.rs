// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut ans = 0;
    for i in 1..vc.len() {
        if vc[i] != vc[i - 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
