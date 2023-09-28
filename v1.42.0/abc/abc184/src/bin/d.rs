use proconio::input;

// https://atcoder.jp/contests/abc184/submissions/18349765 (tkobayashi0111)
// 想定解は dp[100][100][100] = 0.0 から順に dp[a][b][c] を埋めていく遷移

// ~~硬貨を袋に戻すのに取り出す確率変わらないの？~~
// サンプル 2 は初手で 98 の方取り出せば次はどれ引いても終了するのでああいう式になる
// 袋に戻すと確率が変わるため一度に確率を計算する手法では WA

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    // dp: その硬貨状態に繊維する確率 (**期待値ではない**)
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    dp[a][b][c] = 1.0;
    let mut ans = 0.0;
    for i in a..101 {
        for j in b..101 {
            for k in c..101 {
                // println!("({:3}, {:3}, {:3}) に到達する確率: {}", i, j, k, dp[i][j][k]);
                if (i == 100 && j != 100 && k != 100) || (i != 100 && j == 100 && k != 100) || (i != 100 && j != 100 && k == 100) {
                    // println!("+= {}", (i + j + k - a - b - c) as f64 * dp[i][j][k]);
                    ans += (i + j + k - a - b - c) as f64 * dp[i][j][k];
                    continue;
                }

                if i != 100 {
                    dp[i + 1][j][k] += dp[i][j][k] * i as f64 / (i + j + k) as f64;
                }
                if j != 100 {
                    dp[i][j + 1][k] += dp[i][j][k] * j as f64 / (i + j + k) as f64;
                }
                if k != 100 {
                    dp[i][j][k + 1] += dp[i][j][k] * k as f64 / (i + j + k) as f64;
                }
            }
        }
    }

    println!("{}", ans);
}
