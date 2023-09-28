// :fu: 21-02 解けたい
// WA: `UNUSED` が小さすぎた

use proconio::input;
use proconio::marker::Chars;

const UNUSED: i64 = 9_999_999_999_999;

fn main() {
    input! {
        n: usize,
        s: Chars,
        cn: [i64; n],
        dn: [i64; n],
    }

    // dp[i][j]: i 文字目まで見終わった時点で閉じていない左括弧が残り j 個
    let mut dp = vec![vec![UNUSED; n + 1]; n + 1];
    dp[0][0] = 0;
    for (i, ss) in s.iter().enumerate() {
        for j in 0..n {
            if dp[i][j] == UNUSED {
                continue;
            }

            match *ss {
                '(' => {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j]);
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dn[i]);
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j] + cn[i]);
                    }
                }
                ')' => {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j] + cn[i]);
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dn[i]);
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j]);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{}", dp[n][0]);
}
