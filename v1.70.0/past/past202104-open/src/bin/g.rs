use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcm: [(usize, usize, usize); m],
        xq: [usize; q],
    }

    // (cost, city)
    let mut edges = vec![vec![]; n];
    for abc in &abcm {
        let a = abc.0 - 1;
        let b = abc.1 - 1;
        let c = abc.2;
        edges[a].push((c, b));
        edges[b].push((c, a));
    }

    let mut bts = BTreeSet::new();
    for e in &edges[0] {
        bts.insert(*e);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut ans = 1;
    for x in xq {
        let mut next_edges = vec![];
        while let Some(cur) = bts.pop_first() {
            if cur.0 > x {
                next_edges.push(cur);
                break;
            }

            if visited[cur.1] {
                continue;
            }

            visited[cur.1] = true;
            ans += 1;
            for e in &edges[cur.1] {
                next_edges.push(*e);
            }
        }

        for e in next_edges {
            if !visited[e.1] {
                bts.insert(e);
            }
        }

        println!("{ans}");
    }
}
