// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 111 == 0 {
        println!("{}", n);
    } else {
        let d = n / 111;
        println!("{}", 111 * (d + 1));
    }
}
