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
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..k {
            ans += (i + 1) * 100 + j + 1;
        }
    }
    println!("{}", ans);
}
