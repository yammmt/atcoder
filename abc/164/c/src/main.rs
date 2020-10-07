// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
        s: [String; n],
    }

    let mut hs = HashSet::new();
    for i in s {
        hs.insert(i);
    }
    println!("{}", hs.len());
}
