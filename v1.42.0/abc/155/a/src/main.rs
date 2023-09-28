// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8,
    }
    if (a == b && a != c) || (a ==c && a != b) || (b == c && b != a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
