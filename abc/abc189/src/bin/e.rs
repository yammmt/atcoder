// セグ木？
// 回転行列もつと誤差が

// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
        m: usize,
    }

    let mut vmv = vec![vec![(1, 0), (0, 1)]; n + 1];
    for _ in 0..m {
        input! {
            op: usize,
        }

        if op > 2 {
        } else {
        }

    }

    input! {
        q: usize,
        abq: [(usize, usize); q],
    }
    for ab in &abq {
        println!(
            "{} {}",
            xyn[ab.0 - 1].0 * vmv[ab.1 - 1][0].0 + xyn[ab.0 - 1].1 * vmv[ab.1 - 1][0].1,
            xyn[ab.0 - 1].0 * vmv[ab.1 - 1][1].0 + xyn[ab.0 - 1].1 * vmv[ab.1 - 1][1].1
        );
    }
}
