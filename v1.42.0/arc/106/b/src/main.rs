// -*- coding:utf-8-unix -*-

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        bn: [i64; n],
        cdm: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n + 1);
    cdm.iter().for_each(|cd| {
        uf.union(cd.0, cd.1);
    });

    let mut ptotal = vec![0; n + 1];
    (1..n + 1).for_each(|i| ptotal[uf.find(i)] += bn[i - 1] - an[i - 1]);

    println!(
        "{}",
        if ptotal.iter().all(|&p| p == 0) {
            "Yes"
        } else {
            "No"
        }
    );
}
