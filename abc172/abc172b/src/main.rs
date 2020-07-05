// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = s.chars().zip(t.chars()).filter(|&(a, b)| a != b).count();
    println!("{}", ans);
}
