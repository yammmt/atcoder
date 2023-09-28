// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
    }

    if n % 1000 == 0 {
        println!("{}", 0);
    } else {
        println!("{}", 1000 - n % 1000);
    }
}
