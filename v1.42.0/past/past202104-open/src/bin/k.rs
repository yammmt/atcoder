// :fu: 21-06 相応の DP

use proconio::input;

fn main() {
    input! {
        n: usize,
        pun: [(i64, i64); n],
    }

    // dp[i]: 端数 i の最高点
    let mut dp = vec![std::i64::MIN / 3; 100];
    dp[0] = 0;
    for pu in &pun {
        // 枝刈りはあってもなくても結果に影響しなさそう
        // if pu.1 - pu.0 * 4 / 5 < 0 {
        //     continue;
        // }

        let mut cur = dp.clone();
        for i in 0..100 {
            let next_i = i + pu.0 as usize;
            cur[next_i % 100] = cur[next_i % 100].max(dp[i] + pu.1 - pu.0 + next_i as i64 / 100 * 20);
        }

        // println!("{:?}", cur);
        dp = cur;
    }

    println!("{}", dp.iter().max().unwrap());
}
