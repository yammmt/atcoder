// -*- coding:utf-8-unix -*-

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n + 1);
    abm.iter().for_each(|ab| {
        uf.union(ab.0, ab.1);
    });

    let mut grpnum = vec![0; n + 1];
    (1..n + 1).for_each(|i| grpnum[uf.find(i)] += 1);

    println!("{}", grpnum.iter().max().unwrap());
}
