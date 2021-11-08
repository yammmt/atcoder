// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
    }
    let mut hs = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            al: [usize; l],
        }
        hs.insert(al);
    }
    println!("{}", hs.len());
}
