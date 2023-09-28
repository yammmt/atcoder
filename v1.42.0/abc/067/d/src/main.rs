// 29min 1WA (20min) ペナ率高め
// WA: 手元で簡単なケースを試しておく

use proconio::input;
use std::collections::VecDeque;

const INIT: usize = 999_999_999_999;

fn bfs(s: usize, r: usize, edges: &[Vec<usize>]) -> Vec<usize> {
    let mut ret = vec![INIT; edges.len()];
    let mut vdq = VecDeque::new();
    vdq.push_back((s, 0));
    ret[s] = 0;
    while let Some(cur) = vdq.pop_front() {
        for e in &edges[cur.0] {
            if ret[*e] != INIT || *e == r {
                continue;
            }

            vdq.push_back((*e, cur.1 + 1));
            ret[*e] = cur.1 + 1;
        }
    }

    ret
}

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abn {
        edges[ab.0 - 1].push(ab.1 -1);
        edges[ab.1 - 1].push(ab.0 -1);
    }

    let from_fennec = bfs(0, n - 1, &edges);
    let from_snuke = bfs(n - 1, 0, &edges);

    let mut pts_fennec = 0;
    let mut pts_snuke = 0;
    for i in 0..n {
        if from_fennec[i] <= from_snuke[i] {
            pts_fennec += 1;
        } else if from_fennec[i] > from_snuke[i] {
            pts_snuke += 1;
        }
    }

    println!(
        "{}",
        if pts_fennec > pts_snuke {
            "Fennec"
        } else {
            "Snuke"
        }
    );
}
