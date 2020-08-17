// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [char; n],
    }

    if s.iter().any(|&a| a == 'Y') {
        println!("Four");
    } else {
        println!("Three");
    }
}
