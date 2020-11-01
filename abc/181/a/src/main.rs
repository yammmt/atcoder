// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n % 2 == 0 {
        println!("White");
    } else {
        println!("Black");
    }
}
