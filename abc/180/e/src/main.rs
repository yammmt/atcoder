// -*- coding:utf-8-unix -*-

// :fu: :fu: TSP (bitDP) 典型らしい "戻ってくる" の解説が見つからず

use proconio::input;

const UNUSED: usize = std::usize::MAX;

fn move_cost(a: (i64, i64, i64), b: (i64, i64, i64)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + 0.max(b.2 - a.2)) as usize
}

fn main() {
    input! {
        n: usize,
        xyzn: [(i64, i64, i64); n],
    }
    let bit_row_max = (2u32.pow(n as u32) - 1) as usize;

    // dp[訪問済み][現在地] = 最小コスト
    let mut dp = vec![vec![UNUSED; n]; bit_row_max + 1];
    dp[0][0] = 0;
    // [i][j] -> [i + j][k]
    for i in 0..bit_row_max {
        for j in 0..n {
            if dp[i][j] == UNUSED {
                continue;
            }

            for k in 0..n {
                // 訪問済ならスキップ
                if i & (1 << k) > 0 {
                    continue;
                }

                let next_i = i | (1 << k);
                dp[next_i][k] = dp[next_i][k].min(dp[i][j] + move_cost(xyzn[j], xyzn[k]));
            }
        }
    }

    println!("{}", dp[bit_row_max][0]);
}
