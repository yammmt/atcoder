// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        v: i32,
        t: i32,
        s: i32,
        d: i32,
    }

    let drop_start = v * t;
    let drop_end = v * s;

    println!(
        "{}",
        if d < drop_start || drop_end < d {
            "Yes"
        } else {
            "No"
        }
    );
}
