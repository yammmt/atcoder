// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut v = vec![];
    for _ in 0..n {
        input! {
            a: u32,
            b: usize,
        }
        v.push((a, b));
    }
    v.sort();
    let mut current_idx = 0;
    for i in 0..v.len() {
        current_idx += v[i].1;
        if current_idx >= k {
            println!("{}", v[i].0);
            return;
        }
    }
}
