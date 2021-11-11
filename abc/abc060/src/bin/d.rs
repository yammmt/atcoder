// 13min 筋肉解法寄り

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }

    // 重さをインデックスに最大価値を取る DP では重さが最悪 10^9 となり TLE/MLE
    // 価値をインデックスに最小重さを取る DP でも 10^7 の価値を 100 個分ループすると TLE
    // w_1 <= w_i <= w_1 + 3 なる奇妙な制約に目を留めると
    // N 個のものをすべて選択したとして, 取りうる価値の範囲は
    // N*w_1 <= W_max <= (N-1)*(w_1+3) + w_1 **かつ** 値が飛び飛び
    // 配列で DP する部分を HashMap に置き換えてやれば容量が足りそう

    // dp[i]: 重さ i を達成する中で価値最大
    let mut dp = HashMap::new();
    dp.insert(0, 0);
    for wv in &wvn {
        let mut cur = dp.clone();
        for (k, v) in &dp {
            let next_w = k + wv.0;
            let mut next_v = v + wv.1;
            if next_w > w { continue; }

            let cnt = cur.entry(next_w).or_insert(next_v);
            *cnt = *cnt.max(&mut next_v);
        }
        dp = cur;
    }

    let mut ans = 0;
    dp.values().for_each(|&v| ans = ans.max(v));

    println!("{}", ans);
}
