// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u32; n],
    }
    let mut bunbo = 0.0f64;
    for i in &an {
        bunbo += 1.0 / *i as f64;
    }
    println!("{}", 1.0f64 / bunbo);
}
