// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [u64; n],
    }

    let mut x_sorted = x.clone();
    x_sorted.sort();
    for i in 0..n {
        if x[i] >= x_sorted[n / 2] {
            println!("{}", x_sorted[n / 2 - 1]);
        } else {
            println!("{}", x_sorted[n / 2]);
        }
    }
}
