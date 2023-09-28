// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64,
    }

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    let n = h * w;
    let ans = (n + 1) / 2;
    println!("{}", ans as u64);
}
