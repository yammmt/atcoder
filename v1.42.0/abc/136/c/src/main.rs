// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [u64; n],
    }

    for i in 0..n - 1 {
        if !(i > 0 && h[i] == h[i - 1]) {
            h[i] -= 1;
        }
        if h[i] > h[i + 1] {
            println!("No");
            return;
        }
    }
    // println!("{:?}", h);
    println!("Yes");
}
