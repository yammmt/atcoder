// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i64,
        s: String,
        b: i64,
    }
    if s == "+" {
        println!("{}", a + b);
    } else {
        println!("{}", a - b);
    }
}
