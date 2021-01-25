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
        k: i64,
        xn: [i64; n],
    }
    let mut ans = 0;
    for x in &xn {
        let a = 2 * *x;
        let b = 2 * (*x - k).abs();
        ans += a.min(b);
    }
    println!("{}", ans);
}
