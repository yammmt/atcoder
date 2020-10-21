// -*- coding:utf-8-unix -*-

// 10min.

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    // a.sort();
    // println!("{:?}", a);

    let mut hs = HashSet::new();
    for i in &a {
        hs.insert(i);
    }
    // n は奇数
    // n 枚 k 種類
    let ans = if hs.len() % 2 == 0 {
        hs.len() - 1
    } else {
        hs.len()
    };
    println!("{}", ans);
}
