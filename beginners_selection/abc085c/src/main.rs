// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        y: u32,
    }

    for a in 0..n + 1 {
        for b in 0..n - a + 1 {
            let c = n - (a + b);
            if a * 10000 + b * 5000 + c * 1000 == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
