// -*- coding:utf-8-unix -*-

use proconio::input;
// use std::collections::HashMap;

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    for a in 1..n {
        // println!("a = {}, {}", a, (n - 1) / a);
        ans += (n - 1) / a;
    }
    println!("{}", ans);
}
