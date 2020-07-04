// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: char,
    }

    if a <= 'Z' {
        println!("A");
    } else {
        println!("a");
    }
}
