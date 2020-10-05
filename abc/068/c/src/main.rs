// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut from_one = HashSet::new();
    let mut to_n = HashSet::new();
    for i in 0..m {
        if ab[i].0 == 1 {
            from_one.insert(&ab[i].1);
        }
        if ab[i].1 == n {
            to_n.insert(&ab[i].0);
        }
    }

    for i in &from_one {
        if to_n.contains(i) {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}
