// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let ans = (n % k).min((n % k - k).abs());
    println!("{}", ans);
}
