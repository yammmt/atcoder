// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..k + 1{
            if i + j == n {
                break;
            }

            dp[i + j] = dp[i + j].min(dp[i] + (h[i] - h[i + j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
