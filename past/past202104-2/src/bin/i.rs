// 配る貰う DP

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[i64; w]; h],
    }

    // dp[i][j][k]: (i, j) 地点で釣り完了後に k 匹釣った場合の最大スコア
    let mut dp = vec![vec![vec![0; h + w + 1]; w]; h];
    for i in 0..h {
        for j in 0..w {
            // println!("{} {}", i, j);
            let mut new_dp = vec![0; h + w + 1];
            for k in 0..h + w {
                // println!("  k: {}", k);
                // 貰う DP
                if k != 0 && dp[i][j][k] == 0 && i ==0 && j == 0 {
                    // 数が足りず達成不可
                    // println!("  continue");
                    continue;
                }

                new_dp[k + 1] = dp[i][j][k + 1].max(dp[i][j][k] + ahw[i][j]);

                if i > 0 {
                    // TODO: j 直してから
                    // println!("  i > 0");
                    new_dp[k] = new_dp[k].max(dp[i - 1][j][k]);
                    new_dp[k + 1] = new_dp[k + 1].max(dp[i - 1][j][k + 1]);
                    new_dp[k + 1] = new_dp[k + 1].max(dp[i - 1][j][k] + ahw[i][j]);
                }
                if j > 0 {
                    new_dp[k] = new_dp[k].max(dp[i][j - 1][k]);
                    new_dp[k + 1] = new_dp[k + 1].max(dp[i][j - 1][k + 1]);
                    new_dp[k + 1] = new_dp[k + 1].max(dp[i][j - 1][k] + ahw[i][j]);
                }
            }
            // println!("{:?}", new_dp);
            for k in 0..h + w + 1 {
                dp[i][j][k] = new_dp[k];
            }
        }
    }

    // println!("{:?}", dp);
    for a in dp[h - 1][w - 1].iter().skip(1).take(h + w - 1) {
        println!("{}", a);
    }
}
