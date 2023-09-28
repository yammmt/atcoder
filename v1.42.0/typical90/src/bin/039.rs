// :fu: :fu: :fu: 21-07 木 DP
// 水色こんな難しい？ editorial が読めなかった
// https://algo-logic.info/tree-dp/

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abn1: [(usize, usize); n - 1],
    }
    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    let edges = edges;

    let mut ans = 0;
    let mut dp = vec![0; n];
    let mut comes_from = vec![None; n];
    let mut vdq = VecDeque::new();
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_back() {
        // println!("{}", cur);
        if dp[cur] == 0 {
            dp[cur] = 1;
            vdq.push_back(cur);
            for e in &edges[cur] {
                if dp[*e] == 0 {
                    comes_from[*e] = Some(cur);
                    vdq.push_back(*e);
                }
            }
        } else {
            for e in &edges[cur] {
                // 自身の親は除く (汚い)
                if Some(*e) != comes_from[cur] {
                    dp[cur] += dp[*e];
                }
            }
            // println!("  {}", dp[cur]);
            ans += dp[cur] * (n - dp[cur]);
        }
        // println!("{:?}", dp);
    }

    println!("{}", ans);
}
