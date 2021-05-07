use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    let mut dp = vec![vec![vec![0; h + w + 1]; w]; h];
    for i in 0..h {
        for j in 0..w {
            let mut cur_dp = vec![0; h + w];
            // println!("{} {}", i, j);
            for k in 0..h + w {
                if dp[i][j][k] == 0 && k > 0 {
                    // 一匹以上釣って価値 0 となるパターンは存在しない
                    // break した方がちょい速いはず
                    continue;
                }

                let next_v = dp[i][j][k] + ahw[i][j];
                // 釣って移動
                if i + 1 < h {
                    dp[i + 1][j][k + 1] = dp[i + 1][j][k + 1].max(dp[i][j][k + 1]);
                    dp[i + 1][j][k + 1] = dp[i + 1][j][k + 1].max(next_v);
                }
                if j + 1 < w {
                    dp[i][j + 1][k + 1] = dp[i][j + 1][k + 1].max(dp[i][j][k + 1]);
                    dp[i][j + 1][k + 1] = dp[i][j + 1][k + 1].max(next_v);
                }

                // 自身
                cur_dp[k + 1] = dp[i][j][k + 1].max(next_v);
            }
            // println!("  {:?}", cur_dp);
            dp[i][j] = cur_dp;
        }
    }

    for a in dp[h - 1][w - 1].iter().skip(1) {
        println!("{}", a);
    }
}
