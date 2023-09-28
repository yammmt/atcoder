// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i64; d]; n],
    }

    let mut hs = HashSet::new();
    let mut dd = 0;
    // 最大差 (-20 - 20)^2 が 10 点
    while dd * dd <= 16000 {
        let dsq = dd * dd;
        hs.insert(dsq as i64);
        dd += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut cur = 0;
            for k in 0..d {
                cur += (x[i][k] - x[j][k]).pow(2);
            }
            if hs.contains(&cur) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
