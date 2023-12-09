// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
// use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::collections::BTreeSet;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// const DUMMY: usize = usize::MAX / 4;
// const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        mm: u32,
        dd: u32,
        y: u32,
        m: u32,
        d: u32,
    }

    let mut y_nxt = y;
    let mut m_nxt = m;
    let mut d_nxt = d + 1;
    if d_nxt > dd {
        d_nxt = 1;
        m_nxt += 1;
    }
    if m_nxt > mm {
        m_nxt = 1;
        y_nxt += 1;
    }

    println!("{} {} {}", y_nxt, m_nxt, d_nxt);
}
