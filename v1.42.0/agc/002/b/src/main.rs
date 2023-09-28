// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ball_num = vec![1; n + 1];
    let mut hs = HashSet::new();
    hs.insert(1);
    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }

        if hs.contains(&x) {
            hs.insert(y);
            ball_num[x] -= 1;
            ball_num[y] += 1;
            if ball_num[x] == 0 {
                hs.remove(&x);
            }
        } else {
            ball_num[x] -= 1;
            ball_num[y] += 1;
        }
    }
    println!("{}", hs.len());
}
