// 嫌

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
    }

    let mut cleared = HashSet::new();
    let mut p = 2;
    while p * p <= n {
        let mut cur = p * p;
        while cur <= n {
            cleared.insert(cur);
            cur *= p;
        }
        p += 1;
    }

    println!("{}", n as usize - cleared.len());
}
