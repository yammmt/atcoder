// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64,
    }

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    let ans = (h as f64 / 2.0).ceil() * (w as f64 / 2.0).ceil() + (h as f64 / 2.0).floor() * (w as f64 / 2.0).floor();
    println!("{}", ans as u64);
}
