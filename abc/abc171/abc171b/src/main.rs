// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [u32; n],
    }

    p.sort();
    println!("{}", p.iter().take(k).sum::<u32>());
}
