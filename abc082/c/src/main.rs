// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }

    let mut a_counter = HashMap::new();
    for _ in 0..n {
        input! {
            a: u64,
        }
        let counter = a_counter.entry(a).or_insert(0);
        *counter += 1;
    }

    let mut ans = 0;
    for (k, v) in a_counter.iter() {
        // println!("{} {}", k, v);
        if k < v {
            ans += v - k;
        } else if k > v {
            ans += v;
        }
    }
    println!("{}", ans);
}
