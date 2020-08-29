// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let ans = if x % 11 == 0 {
        (x / 11) * 2
    } else if x % 11 < 7 {
        (x / 11) * 2 + 1
    } else {
        (x / 11) * 2 + 2
    };

    println!("{}", ans);
}
