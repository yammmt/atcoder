// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut hn: [u64; n],
    }

    if n <= k {
        println!("0");
        return;
    }

    hn.sort();
    println!("{}", hn.iter().take(n - k).sum::<u64>());
}
