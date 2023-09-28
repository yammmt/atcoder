// :fu: 21-05 DP
// 知らないと無理寄りだが DP 解法ならあるいは

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // dp[i][j]: i/j 文字目からの最長
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if s[i] == s[j] {
                dp[i][j] = dp[i][j].max(dp[i + 1][j + 1] + 1);
            }
        }
    }
    // println!("{:?}", dp);

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans = ans.max(dp[i][j].min(j - i));
        }
    }
    println!("{}", ans);
}
