// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(u64, u64); n],
    }

    let mut dp = vec![std::u64::MAX; h + 1];
    dp[0] = 0;
    // 配る DP (ダメージを与えるのに必要な魔力)
    for iab in &ab {
        // j: ダメージ
        for j in 1..h + 1 {
            // 出発点
            let dp_base_idx = if j < iab.0 as usize {
                0
            } else {
                j - iab.0 as usize
            };

            // println!("dbi: {}", dp_base_idx);
            dp[j] = dp[j].min(dp[dp_base_idx] + iab.1);
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp[h]);
}
