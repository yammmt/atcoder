// -*- coding:utf-8-unix -*-

// :fu: Priority Queue 探す

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abn: [(usize, usize); n],
    }
    abn.sort();

    let mut ans = 0;
    let mut bh = BinaryHeap::new();
    let mut ab_idx = 0;
    for i in 1..m + 1 {
        while ab_idx < n && abn[ab_idx].0 == i {
            bh.push(abn[ab_idx].1);
            ab_idx += 1;
        }
        if bh.is_empty() {
            continue;
        }

        ans += bh.pop().unwrap();
    }

    println!("{}", ans);
}
