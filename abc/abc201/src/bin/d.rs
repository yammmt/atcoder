// :fu: :fu: :fu: 21-05 嫌いな DP

use proconio::input;
use proconio::marker::Chars;

static DUMMY: i64 = std::i64::MIN / 4;

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

    // ややこしいのでエッジケースとして扱う
    if h == 1 && w == 1 {
        println!("Draw");
        return;
    } else if h == 1 {
        let mut tkhs = 0;
        let mut aoki = 0;
        for i in 0..w - 1 {
            if i % 2 == 0 {
                tkhs += phw[0][i + 1];
            } else {
                aoki += phw[0][i + 1];
            }
        }
        println!(
            "{}",
            if tkhs > aoki {
                "Takahashi"
            } else if tkhs < aoki {
                "Aoki"
            } else {
                "Draw"
            }
        );
        return;
    } else if w == 1 {
        let mut tkhs = 0;
        let mut aoki = 0;
        for i in 0..h - 1 {
            if i % 2 == 0 {
                tkhs += phw[i + 1][0];
            } else {
                aoki += phw[i + 1][0];
            }
        }
        println!(
            "{}",
            if tkhs > aoki {
                "Takahashi"
            } else if tkhs < aoki {
                "Aoki"
            } else {
                "Draw"
            }
        );
        return;
    }

    // dp[i][j]: Takahashi がマス (i, j) 以降に取れる最大の得点差
    let mut dp = vec![vec![DUMMY; w]; h];
    dp[h - 1][w - 1] = if (h + w - 2) % 2 == 0 {
        // 最後は Aoki
        -phw[h - 1][w - 1]
    } else {
        // 最後は Tkhs
        phw[h - 1][w - 1]
    };
    // println!("{:?}", dp);

    // 貰う DP
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            // println!("{} {}", i, j);
            if i == h - 1 && j == w - 1 {
                continue;
            }

            if (i + j) % 2 == 1 {
                // ここまでの移動距離が奇数: Takahashi の得点
                if i < h - 1 {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j]);
                }
                if j < w - 1 {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1]);
                }
                if !(i == 0 && j == 0) {
                    dp[i][j] += phw[i][j];
                }
            } else {
                // ここまでの移動距離が偶数: Aoki の得点
                // min 更新の準備でとりあえず大きい値に
                if dp[i][j] == DUMMY {
                    dp[i][j] = -DUMMY;
                }
                if i < h - 1 {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j]);
                }
                if j < w - 1 {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1]);
                }
                if !(i == 0 && j == 0) {
                    dp[i][j] -= phw[i][j];
                }
            }
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        if dp[0][1] > 0 || dp[1][0] > 0 {
            "Takahashi"
        } else if (dp[0][1] < 0 && dp[1][0] < 0)
            || (dp[0][1] == 0 && phw[0][1] == -1 && dp[1][0] < 0)
            || (dp[1][0] == 0 && phw[1][0] == -1 && dp[0][1] < 0)
        {
            "Aoki"
        } else {
            "Draw"
        }
    );
}
