// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            l: i64,
            r: i64,
        }
        if 2 * l > r {
            println!("0");
            continue;
        }

        let anum = r - 2 * l + 1;
        println!(
            "{}",
            anum * (1 + 1 + anum - 1) / 2
        );
    }
}
