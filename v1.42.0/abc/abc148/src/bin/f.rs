// 70.5min (想定 (模範？) 解ではない)

use proconio::input;
use std::collections::VecDeque;

const UNUSED: usize = 9_999_999;

fn bfs(begin: usize, n: usize, edges: &[Vec<usize>]) -> Vec<usize> {
    let mut vdq = VecDeque::new();
    let mut ret = vec![UNUSED; n + 1];
    vdq.push_back((begin, 0));
    ret[begin] = 0;
    while let Some(cur) = vdq.pop_front() {
        for e in &edges[cur.0] {
            if ret[*e] == UNUSED {
                ret[*e] = cur.1 + 1;
                vdq.push_back((*e, cur.1 + 1));
            }
        }
    }
    ret
}

fn main() {
    input! {
        n: usize,
        u: usize,
        v: usize,
        abn: [(usize, usize); n - 1],
    }
    let mut edges = vec![vec![]; n + 1];
    for ab in &abn {
        edges[ab.0].push(ab.1);
        edges[ab.1].push(ab.0);
    }

    let from_u = bfs(u, n, &edges);
    let from_v = bfs(v, n, &edges);

    let mut ans = 1;
    for i in 1..n + 1 {
        if from_u[i] >= from_v[i] {
            continue;
        }

        ans = ans.max(from_v[i]);
    }
    println!("{}", ans - 1);
}
