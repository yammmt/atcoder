// :fu: 21-05 解きたい問題

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        abn: [(usize, usize); n - 1],
    }
    let d = 1_000_000_007;

    let mut edges = vec![vec![]; n];
    for ab in &abn {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    // 距離が短いので全探索しても TLE は回避できると思ったが
    // 例えば頂点 1 に頂点 [2, 100000] が接続されていると TLE
    let mut ans = k;
    let mut visited = vec![false; n];
    let mut vdq = VecDeque::new();
    vdq.push_back((0, 0));
    while let Some(cur) = vdq.pop_back() {
        // println!("{:?}", cur);
        visited[cur.0] = true;
        let mut mul_num = 0;
        if cur.1 == 0 {
            for e in &edges[cur.0] {
                vdq.push_back((*e, 1));
                mul_num += 1;
            }
            // println!("  {}", mul_num);
            for i in 0..mul_num {
                ans = (ans * (k as isize - i - 1).max(0) as usize) % d;
            }
        } else {
            // println!("  {:?}", edges[cur.0]);
            for e in &edges[cur.0] {
                if !visited[*e] {
                    vdq.push_back((*e, cur.1 + 1));
                    mul_num += 1;
                }
            }
            // println!("  {}", mul_num);
            for i in 0..mul_num {
                ans = (ans * (k as isize - i - 2).max(0) as usize) % d;
            }
        }
    }

    println!("{}", ans);
}
