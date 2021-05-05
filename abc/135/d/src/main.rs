// 曰く桁 DP の応用
// https://drken1215.hatenablog.com/entry/2020/04/23/194600

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let d = 1_000_000_007;

    // dp[i][j]: i 桁目まで見て 13 で割った値が j となる数の数
    let mut dp = vec![vec![0; 13]; s.len() + 1];
    dp[0][0] = 1;
    for i in 0..s.len() {
        if s[i] == '?' {
            for j in 0..13 {
                for k in 0..10 {
                    let next_j = (10 * j + k) % 13;
                    dp[i + 1][next_j] = (dp[i + 1][next_j] + dp[i][j]) % d;
                }
            }
        } else {
            for j in 0..13 {
                let next_j = (10 * j + s[i].to_digit(10).unwrap() as usize) % 13;
                dp[i + 1][next_j] = (dp[i + 1][next_j] + dp[i][j]) % d;
            }
        }
    }

    println!("{}", dp[s.len()][5]);
}
