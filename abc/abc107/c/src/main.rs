// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [i64; n],
    }

    let mut ans = std::i64::MAX;
    for i in 0..n - k + 1 {
        // println!("edge: {} {}", x[i], x[i+k-1]);
        let dist = if x[i] * x[i + k - 1] < 0 {
            (x[i] - x[i + k - 1]).abs() + x[i].abs().min(x[i + k - 1].abs())
        } else {
            x[i].abs().max(x[i + k - 1].abs())
        };
        if dist < ans {
            ans = dist;
        }
    }
    println!("{}", ans);
}
