// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        k: i64,
    }

    if k < a {
        println!("{}", k);
    } else if k < a + b {
        println!("{}", a);
    } else {
        println!("{}", a - (k - b - a));
    }
}
