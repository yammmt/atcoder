use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    // 貰う DP
    let mut dp = vec![vec![vec![0; h + w]; w]; h];
    // 初期値を手動で入れねば漸化式が更新されない
    dp[0][0][1] = ahw[0][0];
    for i in 0..h {
        for j in 0..w {
            for k in 1..h + w {
                if i > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k]);
                    dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k - 1] + ahw[i][j]);
                }
                if j > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k]);
                    dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k - 1] + ahw[i][j]);
                }
            }
        }
    }

    for k in 1..h + w {
        println!("{}", dp[h - 1][w - 1][k]);
    }
}
