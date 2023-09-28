// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let mut ans = std::i64::MIN / 2;
    for x in a..b + 1 {
        for y in c..d + 1 {
            ans = ans.max(x - y);
        }
    }
    println!("{}", ans);
}
