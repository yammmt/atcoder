// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        x: Chars,
    }
    let mut ans = vec![];
    for c in &x {
        if *c == '.' {
            break;
        }

        ans.push(*c);
    }
    println!("{}", ans.iter().collect::<String>());
}
