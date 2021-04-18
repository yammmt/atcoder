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
        an: [i64; n],
        bn: [i64; n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += an[i] * bn[i];
    }
    println!(
        "{}",
        if ans == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
