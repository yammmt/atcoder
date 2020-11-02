// -*- coding:utf-8-unix -*-

// 7min

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m],
    }
    let mut tp = Vec::with_capacity(n + 1);
    for i in 0..n + 1 {
        tp.push((m + 1, i));
    }

    for i in 0..m {
        tp[am[i]] = (m - i, am[i]);
    }

    tp.sort();
    for i in 0..tp.len() {
        if tp[i].1 == 0 {
            continue;
        }

        println!("{}", tp[i].1);
    }
}
