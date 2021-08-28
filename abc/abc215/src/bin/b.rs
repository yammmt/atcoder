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
        n: u128,
    }
    let mut ans = 0;
    let mut cur = 1;
    loop {
        if cur > n {
            println!("{}", ans - 1);
            return;
        }
        ans += 1;
        cur *= 2;
    }
}
