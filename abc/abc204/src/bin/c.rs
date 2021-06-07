// C...?

use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
    }

    let mut ans = 0usize;
    for i in 0..n {
        let mut vdq = VecDeque::new();
        let mut hs = HashSet::new();
        vdq.push_back(i);
        hs.insert(i);
        while let Some(cur) = vdq.pop_front() {
            for e in &edges[cur] {
                if !hs.contains(e) {
                    vdq.push_back(*e);
                    hs.insert(*e);
                }
            }
        }
        ans += hs.len();
    }

    println!("{}", ans);
}
