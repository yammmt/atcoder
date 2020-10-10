// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut hs = HashSet::new();
    let mut cans = 0;
    let mut vans = vec![];
    for _ in 0..n {
        input! {
            p: u64,
        }
        hs.insert(p);
        if cans == p {
            // 最小値を更新
            for i in p + 1..200002 {
                if !hs.contains(&i) {
                    cans = i;
                    break;
                }
            }
        }
        vans.push(cans);
    }

    for i in &vans {
        println!("{}", *i);
    }
}
