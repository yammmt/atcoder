use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abn: [(usize, usize); n],
    }
    // 日付昇順
    abn.sort_unstable();

    let mut ans = 0;
    let mut available_jobs = BinaryHeap::new();
    let mut ab_idx = 0;
    // 最終日から順にできる範囲で単価最高の仕事
    // ループ範囲怖い
    for i in 1..=m {
        while ab_idx < n && abn[ab_idx].0 <= i {
            available_jobs.push(abn[ab_idx].1);
            ab_idx += 1;
        }

        if let Some(job) = available_jobs.pop() {
            ans += job;
        }
    }

    println!("{}", ans);
}
