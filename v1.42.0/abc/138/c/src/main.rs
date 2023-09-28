// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut vn: [u32; n],
    }
    vn.sort();
    let mut current = (vn[0] + vn[1]) as f64 / 2.0;
    for i in 2..n {
        current = (current + vn[i] as f64) / 2.0;
    }
    println!("{}", current);
}
