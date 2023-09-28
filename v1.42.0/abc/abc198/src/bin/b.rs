// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        mut n: Chars,
    }
    n.reverse();

    let mut vn = vec![];
    let mut i = 0;
    while i < n.len() && n[i] == '0' {
        i += 1;
    }
    if i == n.len() {
        println!("Yes");
        return;
    }

    while i < n.len() {
        vn.push(n[i]);
        i += 1;
    }

    let mut rvn = vn.clone();
    rvn.reverse();
    println!("{}", if vn == rvn { "Yes" } else { "No" });
}
