// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        na: usize,
        nb: usize,
    }
    let mut hsa = HashSet::new();
    for _ in 0..na {
        input! {
            n: usize,
        }
        hsa.insert(n);
    }
    let mut hsb = HashSet::new();
    for _ in 0..nb {
        input! {
            n: usize,
        }
        hsb.insert(n);
    }

    let mut common_num = 0;
    for k in &hsa {
        if hsb.contains(&k) {
            common_num += 1;
        }
    }
    println!("{}", common_num as f64 / ((hsa.len() + hsb.len() - common_num) as f64));
}
