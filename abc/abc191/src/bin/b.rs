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
        x: i32,
        an: [i32; n],
    }
    let mut ans = vec![];
    for a in &an {
        if *a != x {
            ans.push(*a);
        }
    }
    for (i, a) in ans.iter().enumerate() {
        print!("{}", *a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
