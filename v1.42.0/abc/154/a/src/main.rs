// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
        a: u32,
        b: u32,
        u: String,
    }
    if s == u {
        println!("{} {}", a - 1, b);
    } else if t == u {
        println!("{} {}", a, b - 1);
    } else {
        println!("{} {}", a, b);
    }
}
