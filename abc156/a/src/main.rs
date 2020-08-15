// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        r: u16,
    }

    if n < 10 {
        println!("{}", r + 100 * (10 - n));
    } else {
        println!("{}", r);
    }
}
