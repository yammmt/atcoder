// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }

    let mut hm = HashMap::new();
    for _ in 0..n {
        input! {
            a: String,
        }
        let mut b: Vec<char> = a.chars().collect();
        b.sort();
        let counter = hm.entry(b).or_insert(0 as u64);
        *counter += 1;
    }

    let mut ans: u64 = 0;
    for (_, v) in hm.iter() {
        if v % 2 == 0 {
            ans += ((v / 2) * (v - 1)) as u64;
        } else {
            ans += ((v - 1) / 2 * v) as u64;
        }
    }
    println!("{}", ans);
}
