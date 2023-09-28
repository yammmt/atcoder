// :fu: 21-09 言語が辛い (個人の感想

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
        cxq: [(usize, usize); q],
    }

    // (始点, 終点) で区間を取れれば BTreeSet で処理できる？
    // 問われた点より小さな始点をもつ区間値を出力すれば良い
    // "クエリを処理する時点で切られていない" ので面倒な条件を考えなくても良い
    let mut bts: BTreeSet<(usize, usize)> = BTreeSet::new();
    bts.insert((0, l));
    for cx in &cxq {
        // println!("{:?}", bts);
        // println!("  {:?}", cx);
        if cx.0 == 1 {
            // 分割して再挿入
            let cur = *bts.range(..(cx.1, l + 1)).next_back().unwrap();
            let calibrated_len = cx.1 - cur.0;
            // println!("  {:?}", cur);
            // println!("  {}", calibrated_len);
            bts.insert((cur.0, calibrated_len));
            bts.insert((cx.1, cur.1 - calibrated_len));
            bts.remove(&cur);
        } else {
            // 出力
            let ans = bts.range(..(cx.1, l + 1)).next_back().unwrap();
            println!("{}", ans.1);
        }
    }
}
