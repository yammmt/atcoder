// わからん

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    // dp[i][j]: 文字列 S の区間 [i, j) を取り出した部分文字列にて "iwi" を削除できる最大回数
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for w in 2..=n {
        for i in 0..n {
            // j: 区間の右端
            let j = i + w;
            if j > n {
                break;
            }

            if s[i] == 'i' && s[j - 1] == 'i' {
                for k in i + 1..j - 1 {
                    // k を境に中間の文字列がすべて削除できるか？
                    if s[k] == 'w'
                        && dp[i + 1][k] * 3 == k - i - 1
                        && dp[k + 1][j - 1] * 3 == j - k - 2
                    {
                        dp[i][j] = (j - i) / 3;
                    }
                }
            }

            // 境前後の最大値を見ているだけ
            for k in i + 1..j - 1 {
                dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
