// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if (a <= c && c <= b) || (c <= a && a <= d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
