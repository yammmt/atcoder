// thanks: https://scrapbox.io/pocala-kyopro/N_-_Slimes

use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut cusum = vec![0; n + 1];
    for i in 0..n {
        cusum[i + 1] = cusum[i] + an[i];
    }

    // dp[i][j] = 区間 [i, j) での最小コスト
    // dp[i][i + 1] は区間 [i, i+1) = [i] となりコスト 0
    let mut dp = vec![vec![DUMMY; n + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][i] = 0;
    }
    for i in 0..n {
        dp[i][i + 1] = 0;
    }

    // w: 横幅
    for w in 2..=n {
        for i in 0..=n - w {
            for j in i + 1..i + w {
                dp[i][i + w] = dp[i][i + w].min(dp[i][j] + dp[j][i + w] + cusum[i + w] - cusum[i]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
