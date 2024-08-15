use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // dp[i][j]: i 回目 (0-origin) に手 j を出した場合の最多勝数
    //           j は小さい順に R, P, S

    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        match s[i] {
            'R' => {
                // R
                dp[i + 1][0] = dp[i][1].max(dp[i][2]);
                // P
                dp[i + 1][1] = dp[i][0].max(dp[i][2]) + 1;
            }
            'P' => {
                // P
                dp[i + 1][1] = dp[i][0].max(dp[i][2]);
                // S
                dp[i + 1][2] = dp[i][0].max(dp[i][1]) + 1;
            }
            'S' => {
                // R
                dp[i + 1][0] = dp[i][1].max(dp[i][2]) + 1;
                // S
                dp[i + 1][2] = dp[i][0].max(dp[i][1]);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp[n][0].max(dp[n][1].max(dp[n][2])));
}
