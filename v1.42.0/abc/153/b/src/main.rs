// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: u64,
        n: usize,
        an: [u64; n],
    }
    if an.iter().sum::<u64>() >= h {
        println!("Yes");
    } else {
        println!("No");
    }
}
