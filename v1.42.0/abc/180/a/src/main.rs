// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        a: u16,
        b: u16,
    }
    println!("{}", n - a + b);
}
