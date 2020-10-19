// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [f64; n],
    }
    let mut mh = 0.0f64;
    let mut eu = 0.0f64;
    let mut ch = 0.0f64;
    for i in &x {
        mh += i.abs();
        eu += *i * *i;
        ch = ch.max(i.abs());
    }
    eu = eu.sqrt();
    println!("{}", mh);
    println!("{}", eu);
    println!("{}", ch);
}
