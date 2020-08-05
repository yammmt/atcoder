// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let ans: u64 = (1..n + 1).filter(|i| i % 3 != 0 && i % 5 != 0).sum();
    println!("{}", ans);
}
