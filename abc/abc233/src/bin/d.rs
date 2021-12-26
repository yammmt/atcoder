// :fu: :fu: :fu: :fu: 21-12 とても苦手
// WA: オーバーフロー
// 他の回答 cusum - k ばかりだが, 左から二回数えるこの方針でも通る

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: i64,
        an: [i64; n],
    }

    let mut cusum = HashMap::new();
    let mut cur = 0;
    for a in &an {
        cur += *a;
        let cnt = cusum.entry(cur).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0i64;
    let mut cur = 0;
    for &a in &an {
        // a を始点として == k となる数列を数える
        // ここまでの累積和分の下駄を履く
        ans += cusum.get(&(k + cur)).unwrap_or(&0);

        // 自身までの累積和は全体から引く
        // max をつけ忘れても通るが累積和マップ値 < 0 となり嘘解に見える
        cur += a;
        let cnt = cusum.entry(cur).or_insert(0);
        *cnt = (*cnt - 1).max(0);
    }
    // println!("{:?}", cusum);
    // ひっかからない
    assert!(cusum.iter().all(|(_k, v)| *v == 0));

    println!("{}", ans);
}
