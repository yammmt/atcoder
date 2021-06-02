// 7.5min
// 部分点の制約より Priority Queue なしでも解ける (想定解)

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        abn: [(usize, usize); n],
    }

    let mut part_used = vec![false; n];
    let mut bh = BinaryHeap::new();
    for (i, ab) in abn.iter().enumerate() {
        bh.push((ab.1, i));
    }
    // println!("{:?}", bh);

    let mut ans = 0;
    for _ in 0..k {
        let cur = bh.pop().unwrap();
        ans += cur.0;
        if !part_used[cur.1] {
            bh.push((abn[cur.1].0 - abn[cur.1].1, cur.1));
            part_used[cur.1] = true;
        }
    }

    println!("{}", ans);
}
