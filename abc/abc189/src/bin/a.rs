// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        c: Chars,
    }

    println!(
        "{}",
        if c[0] == c[1] && c[1] == c[2] {
            "Won"
        } else {
            "Lost"
        }
    );
}
