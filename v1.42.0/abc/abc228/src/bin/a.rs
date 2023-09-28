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
        s: i32,
        mut t: i32,
        mut x: i32,
    }
    if t < s {
        t += 24;
        if x < s {
            x += 24;
        }
    }
    println!("{}", if s <= x && x < t { "Yes" } else { "No" });
}
