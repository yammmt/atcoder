// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut abn: [(usize, usize); n],
    }
    abn.sort();

    let mut ab_idx = 0;
    let mut ans = 0;
    let mut money = k;
    while money > 0 {
        ans += money;
        money = 0;

        while ab_idx < n && abn[ab_idx].0 <= ans {
            money += abn[ab_idx].1;
            ab_idx += 1;
        }
    }
    println!("{}", ans);
}
