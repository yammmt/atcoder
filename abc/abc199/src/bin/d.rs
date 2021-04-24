// NOT WORK なんだこの D 問題

use proconio::input;
use std::collections::VecDeque;

// 3^20 > 10^9
// グラフを連結成分に分解して、その中で一点を決めた際に
// 一頂点から三点以上に線が惹かれていると解は 0

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    for e in &edges {
        if e.len() > 2 {
            println!("0");
            return;
        }
    }

    let mut visited = vec![false; n];
    let mut ans = 1;
    for i in 0..n {
        if visited[i] {
            continue;
        }

        // let mut vdq = VecDeque::new();
        // vdq.push_back();
    }

    println!("{}", ans);
}
