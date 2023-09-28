// :fu: :fu: :fu: 21-03 LCS

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        bm: [i64; m],
    }

    // dp[i][j]: an/bm を i/j 番目まで見た場合の最小値
    let mut dp = vec![vec![std::usize::MAX / 2; m + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][0] = i;
    }
    for i in 0..m + 1 {
        dp[0][i] = i;
    }

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            // A を消す
            dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            // B を消す
            dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            // A/B を残す
            dp[i][j] = dp[i][j].min(
                if an[i - 1] == bm[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    dp[i - 1][j - 1] + 1
                }
            );
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[n][m]);
}
