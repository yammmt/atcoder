// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        s: String,
    }

    let vc = s.chars().collect::<Vec<char>>();

    for i in 0..vc.len() {
        if (i % 2 == 0 && vc[i].is_ascii_uppercase())
            || (i % 2 == 1 && vc[i].is_ascii_lowercase())
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
