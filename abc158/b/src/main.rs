// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }

    let ans = a * (n / (a + b)) + (n % (a + b)).min(a);
    println!("{}", ans);
}
