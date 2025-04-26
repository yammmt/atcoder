use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    const DUMMY: usize = usize::MAX / 3;

    input! {
        n: usize,
        s: Chars,
        cn: [usize; n],
        dn: [usize; n],
    }

    // dp[i][j]: 先頭 i 文字分について, 左括弧の数が右より j 個多い状態にするための最小コスト
    // 最後の場合分けがめんどいのでどっちも +1
    let mut dp = vec![vec![DUMMY; n + 1]; n + 1];
    dp[0][0] = 0;

    for (i, c) in s.iter().enumerate() {
        match c {
            '(' => {
                for j in 0..n {
                    if dp[i][j] == DUMMY {
                        continue;
                    }

                    // そのまま
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j]);
                    // ) に変える
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j] + cn[i]);
                    }
                    // 消す
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dn[i]);
                }
            }
            ')' => {
                for j in 0..n {
                    if dp[i][j] == DUMMY {
                        continue;
                    }

                    // そのまま
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j]);
                    }
                    // ( に変える
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j] + cn[i]);
                    // 消す
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dn[i]);
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp[n][0]);
}
