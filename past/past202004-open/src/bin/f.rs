use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut abn: [(usize, i64); n],
    }

    abn.sort_unstable();
    let mut ab_idx = 0;
    let mut bh = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..n {
        while ab_idx < n && abn[ab_idx].0 - 1 <= i {
            bh.push(abn[ab_idx].1);
            ab_idx += 1;
        }

        ans += bh.pop().unwrap();
        println!("{}", ans);
    }
}
