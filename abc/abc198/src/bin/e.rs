// :fu: 21-04

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        ci: [usize; n],
        abn1: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    // println!("{:?}", edges);

    let mut visited = vec![false; n];
    let mut appeared = vec![false; 100_001];
    let mut vdq = VecDeque::new();
    appeared[ci[0] - 1] = true;
    visited[0] = true;
    vdq.push_back((0, appeared));
    let mut ans = vec![1];
    while let Some(cur) = vdq.pop_front() {
        // println!("{:?}", cur);
        for e in &edges[cur.0] {
            if visited[*e] {
                continue;
            }
            // println!("  {}", e);
            // println!("  {:?}", ci);
            // println!("  {:?}", cur.1);
            if !cur.1[ci[*e] - 1] {
                ans.push(e + 1);
            }

            visited[*e] = true;
            let mut nextban = cur.1.clone();
            nextban[ci[*e] - 1] = true;
            vdq.push_back((*e, nextban));
        }
    }

    ans.sort_unstable();
    for a in &ans {
        println!("{}", a);
    }
}
