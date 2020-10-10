// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }
    println!("{}", a.iter().sum::<u64>() - n);
}
