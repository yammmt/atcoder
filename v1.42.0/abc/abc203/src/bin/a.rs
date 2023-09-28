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
        a: u8,
        b: u8,
        c: u8,
    }
    println!(
        "{}",
        if a != b && b != c && c != a {
            0
        } else if a == b {
            c
        } else if a == c {
            b
        } else {
            a
        }
    );
}
