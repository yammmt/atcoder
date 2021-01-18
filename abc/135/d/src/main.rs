// -*- coding:utf-8-unix -*-

// :fu:
// 曰く桁 DP の応用
// https://drken1215.hatenablog.com/entry/2020/04/23/194600

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let d = 10u64.pow(9) + 7;

    let mut dp = vec![vec![0; 13]; s.len() + 1];
    // 0 % 13 == 0
    dp[0][0] = 1;
    for (i, c) in s.iter().enumerate() {
        for j in 0..13 {
            if *c == '?' {
                for k in 0..10 {
                    let jj = ((10 * j) + k) % 13;
                    dp[i + 1][jj] = (dp[i + 1][jj] + dp[i][j]) % d;
                }
            } else {
                let n = (*c as u8 - b'0') as usize;
                let jj = ((10 * j) + n) % 13;
                dp[i + 1][jj] = (dp[i + 1][jj] + dp[i][j]) % d;
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[s.len()][5]);
}
