// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l: u16,
    }
    let n = l as f64 / 3.0;
    println!("{}", n * n * n);
}
