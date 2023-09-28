use proconio::input;
use std::collections::VecDeque;

const UNVISITED: usize = std::usize::MAX / 2;

fn bfs(start: usize, edges: &[Vec<usize>]) -> Vec<usize> {
    let mut dist = vec![UNVISITED; edges.len()];
    let mut vdq = VecDeque::new();
    vdq.push_back((start, 0));
    while let Some(cur) = vdq.pop_front() {
        dist[cur.0] = cur.1;
        for e in &edges[cur.0] {
            if dist[*e] == UNVISITED {
                vdq.push_back((*e, cur.1 + 1));
            }
        }
    }
    dist
}

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

    let dist_from_0 = bfs(0, &edges);

    let mut longest = (0, 0);
    for (i, d) in dist_from_0.iter().enumerate() {
        if *d > longest.1 {
            longest = (i, *d);
        }
    }

    let dist_from_l = bfs(longest.0, &edges);

    let mut ans = 0;
    dist_from_l.iter().for_each(|d| ans = ans.max(*d));

    println!("{}", ans + 1);
}
