use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

static MOD: usize = 998_244_353;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    // dp[i][j]: i 文字目完了時点で左括弧の数が j 個
    //           左括弧数負はその時点でカッコ列にならないので捨てる
    let mut dp = vec![vec![0; s.len() + 1]; s.len() + 1];
    dp[0][0] = 1;
    for (i, c) in s.iter().enumerate() {
        match c {
            '(' => {
                for j in 0..s.len() {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % MOD;
                }
            }
            ')' => {
                // +1 あってもなくても同じはず (全部右括弧でないとそうならない)
                for j in 1..s.len() {
                    dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % MOD;
                }
            }
            '?' => {
                for j in 0..s.len() {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % MOD;
                }
                for j in 1..s.len() {
                    dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % MOD;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp[s.len()][0]);
}
