// -*- coding:utf-8-unix -*-

// 9min 1WA

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }
    if a == 0 {
        println!("{}", b / x + 1);
    } else {
        println!("{}", b / x - (a - 1) / x);
    }
}
