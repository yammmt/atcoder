// < 10min
// UnionFind やるだけ

use petgraph::unionfind::UnionFind;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    if m == 0 {
        println!("{}", n - 1);
        return;
    }

    input! {
        abm: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    abm.iter().for_each(|ab| { uf.union(ab.0 - 1, ab.1 - 1); });
    let mut hs = HashSet::new();
    (0..n).for_each(|i| { hs.insert(uf.find(i)); });

    println!("{}", hs.len() - 1);
}
