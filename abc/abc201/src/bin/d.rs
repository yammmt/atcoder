// :fu: :fu: :fu: 21-05 嫌いな DP
// WA: DP 遷移式が逆 どうしてこれでサンプルが通ってしまうのか (二択だから)

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        ah: [Chars; h],
    }

    let mut phw = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            phw[i][j] = if ah[i][j] == '+' {
                1
            } else {
                -1
            };
        }
    }
    let phw = phw;

    // 最後は必ず右下で終わる
    // 自身の点数を稼ぐには自身が "+" に進んで相手を "-" に進ませる他ない

    // dp[i][j]: マス (i, j) からの移動後にて Takahashi が取得できる最大の得点差
    let mut dp = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                continue;
            }

            if (i + j) % 2 == 0 {
                // tkhs
                dp[i][j] = std::i64::MIN / 2;
                if i < h - 1 {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j] + phw[i + 1][j]);
                }
                if j < w - 1 {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1] + phw[i][j + 1]);
                }
            } else {
                // aoki
                dp[i][j] = std::i64::MAX / 2;
                if i < h - 1 {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j] - phw[i + 1][j]);
                }
                if j < w - 1 {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1] - phw[i][j + 1]);
                }
            }
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        match dp[0][0] {
            d if d > 0 => "Takahashi",
            d if d < 0 => "Aoki",
            _ => "Draw",
        }
    );
}
