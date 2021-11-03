// 62min (WA: 45.5min)
// WA: (44 49), 桁が 4/9 だった場合の更新処理に抜け

// 古いのでテストケース非公開
// 桁 DP はバグらせる

use proconio::input;

// n 以下の "使える" 数を返す
// WA: n = 43
fn degree_dp(n: usize) -> usize {
    // println!("n: {}", n);
    if n == 0 {
        return 1;
    }

    let mut nn = n;
    let mut degrees = vec![];
    while nn > 0 {
        degrees.push(nn % 10);
        nn /= 10;
    }
    degrees.reverse();

    // dp[i][j][k]: 上から見て i 桁目の数字が j である使える数の個数
    //              k = 0: n 以下であることが未確定
    let mut dp = vec![vec![vec![0; 2]; 10]; degrees.len()];
    // 初回だけ分けておく
    for j in 0..degrees[0] {
        if j == 4 || j == 9 {
            continue;
        }

        dp[0][j][1] = 1;
    }
    if degrees[0] != 4 && degrees[0] != 9 {
        dp[0][degrees[0]][0] = 1;
    }

    // 配る桁 DP
    for i in 0..degrees.len() - 1 {
        for j in 0..10 {
            // i 桁目が j である場合に i + 1 桁目を更新する
            if j == 4 || j == 9 {
                // 不要のはずだが
                continue;
            }

            // i + 1 桁目が jj であるとする
            // k = 1: n 以下であることが確定 (すべての数が使える)
            for jj in 0..10 {
                if jj == 4 || jj == 9 {
                    continue;
                }

                dp[i + 1][jj][1] += dp[i][j][1];
            }

            // k = 0: n 以下であることが未確定
            for jj in 0..degrees[i + 1] {
                if jj == 4 || jj == 9 {
                    continue;
                }

                dp[i + 1][jj][1] += dp[i][j][0];
            }
            if degrees[i + 1] != 4 && degrees[i + 1] != 9 {
                dp[i + 1][degrees[i + 1]][0] = dp[i][degrees[i]][0];
            }
        }
    }
    // println!("  {:?}", dp);

    let mut ret = 0;
    for j in 0..10 {
        ret += dp[degrees.len() - 1][j][1];
    }
    ret += dp[degrees.len() - 1][degrees[degrees.len() - 1]][0];
    // println!("  {}", ret);
    ret
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    // 各桁に対し使える数が 8 通りであるので, 最大数より 8^x を使って数を出すと A/B の端数分が難しい
    // 桁 DP しよう
    // 範囲全体から使える数を引いたものが答え
    println!("{}", (b - a + 1) - (degree_dp(b) - degree_dp(a - 1)));
}
