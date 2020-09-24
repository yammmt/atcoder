// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let a = a - 1;
    let b_leap = (b / 4) - (b / 100) + (b / 400);
    let a_leap = (a / 4) - (a / 100) + (a / 400);
    // println!("{} - {}", b_leap, a_leap);
    println!("{}", b_leap - a_leap);
}
