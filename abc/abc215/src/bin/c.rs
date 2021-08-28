use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let slen = s.len();
    let mut ans = vec![];
    for perm in (0..slen).permutations(slen) {
        let mut cur = vec![];
        for p in &perm {
            cur.push(s[*p]);
        }
        ans.push(cur.iter().collect::<String>());
    }

    ans.sort_unstable();
    ans.dedup();
    println!("{}", ans[k - 1]);
}
