// 16min

use proconio::input;
use std::collections::VecDeque;

const DUMMY: usize = std::usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m],
        s: usize,
        t: usize,
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        // 有効辺
        edges[uv.0 - 1].push(uv.1 - 1);
    }
    let edges = edges;

    // costs[i][j]: 頂点 i に対し状態 j で辿り着くまでの最小コスト
    // j: けん - けん - ぱ で 0 - 1 - 2
    let mut costs = vec![vec![DUMMY; 3]; n];
    let mut vdq = VecDeque::new();
    // (頂点, 状態)
    vdq.push_back((s - 1, 2));
    costs[s - 1][2] = 0;
    while let Some(cur) = vdq.pop_front() {
        let next_state = (cur.1 + 1) % 3;
        for e in &edges[cur.0] {
            if costs[*e][next_state] != DUMMY {
                continue;
            }

            vdq.push_back((*e, next_state));
            costs[*e][next_state] = costs[cur.0][cur.1] + 1;
        }
    }

    println!(
        "{}",
        if costs[t - 1][2] == DUMMY {
            -1
        } else {
            (costs[t - 1][2] as isize + 1) / 3
        }
    );
}
