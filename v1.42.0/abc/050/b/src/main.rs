// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u64; n],
        m: usize,
        px: [(u64, u64); m],
    }
    for i in 0..m {
        let mut ans = 0;
        for j in 0..n {
            ans += if (j + 1) as u64 == px[i].0 {
                px[i].1
            } else {
                t[j]
            };
        }
        println!("{}", ans);
    }
}
