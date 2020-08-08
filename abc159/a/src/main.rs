// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        m: u16,
    }

    println!("{}", (n * (n - 1) / 2) + (m * (m - 1)) / 2);
}
