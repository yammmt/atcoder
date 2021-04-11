// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64,
    }

    let z2 = x * x + y * y;
    let r2 = r * r;
    if z2 < r2 {
        println!("2");
        return;
    }

    let ans2 = (z2 + r2 - 1) / r2;
    let mut ans = 1;
    while ans * ans < ans2 {
        ans += 1;
    }
    println!("{}", ans);
}
