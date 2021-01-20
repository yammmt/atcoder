// :fu: :fu: :fu: ナップザック DP 亜種
// 類題 https://qiita.com/drken/items/dc53c683d6de8aeacf5a#%E9%A1%9E%E9%A1%8C-3

use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        k: usize,
        // 幅 0 重要度 1
        abn: [(usize, i64); n],
    }

    // dp[a][b][c]: a 枚目まで見て b 枚以下使用時に幅 c 以内での重要度最大値
    // a 枚目を見る、いわゆる配る DP
    let mut dp = vec![vec![vec![0; w + 1]; k + 1]; n + 1];
    for a in 0..n {
        for b in 0..k + 1 {
            for c in 0..w + 1 {
                // 枚数/幅は直前の値が現在の数字より良い可能性があり
                // k/w に対するループを回しきらねばならない
                dp[a + 1][b][c] = dp[a + 1][b][c].max(dp[a][b][c]);

                // b < k: k 枚使い切ったなら更新不可
                // c >= abn[a].0: a を採用すると幅制約を超過する
                if b < k && c >= abn[a].0 {
                    dp[a + 1][b + 1][c] = dp[a + 1][b + 1][c].max(
                        dp[a][b][c - abn[a].0] + abn[a].1
                    );
                }
            }
        }
        // println!("{:?}", dp);
    }

    println!("{}", dp[n][k][w]);
}
