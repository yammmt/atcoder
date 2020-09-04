// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    if c - a - b < 0 {
        println!("No");
        return;
    }

    if (c - a - b).pow(2) > 4 * a * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
