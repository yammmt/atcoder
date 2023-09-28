// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        d: u32,
        n: usize,
    }
    let base = if d == 0 {
        1
    } else {
        100u32.pow(d)
    };
    if n == 100 {
        println!("{}", base * 101);
    } else {
        println!("{}", base * n as u32);
    }
}
