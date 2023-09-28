// 16min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrdm: [(usize, usize, i64); m],
    }

    // グラフの連結成分を取って任意の一人の位置を 0 で決め打ちして代入し続ける
    // 矛盾したら No
    let mut edges = vec![vec![]; n];
    for lrd in &lrdm {
        edges[lrd.0 - 1].push((lrd.1 - 1, lrd.2));
        edges[lrd.1 - 1].push((lrd.0 - 1, -lrd.2));
    }
    let edges = edges;

    let mut pos = vec![None; n];
    for i in 0..n {
        if pos[i].is_some() {
            continue;
        }

        let mut q = VecDeque::new();
        pos[i] = Some(0);
        q.push_back(i);
        while let Some(cur) = q.pop_front() {
            for e in &edges[cur] {
                if pos[e.0] != None {
                    continue;
                }

                pos[e.0] = Some(pos[cur].unwrap() + e.1);
                q.push_back(e.0);
            }
        }
    }
    // println!("{:?}", pos);

    for lrd in &lrdm {
        if pos[lrd.1 - 1].unwrap() - pos[lrd.0 - 1].unwrap() != lrd.2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
