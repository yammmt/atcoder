// 強連結成分分解 (SCC)
// WA: グラフは連結とは限らない + 同じ頂点を何度も探索していた

use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abm: [(usize, usize); m],
    }
    // 多重辺を潰す
    abm.sort();
    abm.dedup();
    // println!("{:?}", abm);

    let mut edge_forward = vec![vec![]; n];
    let mut edge_back = vec![vec![]; n];
    for ab in &abm {
        edge_forward[ab.0 - 1].push(ab.1 - 1);
        edge_back[ab.1 - 1].push(ab.0 - 1);
    }
    // println!("{:?}", edge_forward);
    // println!("{:?}", edge_back);

    let mut order = vec![];
    let mut order_saved = vec![false; n];
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut vdq = VecDeque::new();
        vdq.push_back(start);
        while let Some(cur) = vdq.pop_back() {
            if visited[cur] {
                if !order_saved[cur] {
                    // 辺のソートをコメントアウトしつつ
                    // 1 -> 2, 1 -> 3, 2 -> 3 と有効辺を貼った場合に
                    // 3 が二度 push されてバグる
                    order_saved[cur] = true;
                    order.push(cur);
                }
                continue;
            }

            visited[cur] = true;
            vdq.push_back(cur);
            for e in &edge_forward[cur] {
                if !visited[*e] {
                    vdq.push_back(*e);
                }
            }
        }
    }
    assert!(visited.iter().all(|&b| b));
    assert_eq!(order.len(), n);

    order.reverse();
    visited = vec![false; n];
    let mut ans = 0;
    for start in &order {
        if visited[*start] {
            continue;
        }

        let mut vdq = VecDeque::new();
        let mut hs = HashSet::new();
        vdq.push_back(*start);
        while let Some(cur) = vdq.pop_back() {
            hs.insert(cur);
            if visited[cur] {
                continue;
            }

            visited[cur] = true;
            for e in &edge_back[cur] {
                if !visited[*e] {
                    vdq.push_back(*e);
                }
            }
        }
        if hs.len() > 1 {
            ans += (hs.len() * (hs.len() - 1)) / 2;
        }
    }
    assert!(visited.iter().all(|&b| b));

    println!("{}", ans);
}
