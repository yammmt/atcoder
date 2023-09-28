// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut hs = HashSet::new();
    for i in 0..n {
        if hs.contains(&a[i]) {
            ans += 1;
        } else {
            hs.insert(&a[i]);
        }
    }
    println!("{}", ans);
}
