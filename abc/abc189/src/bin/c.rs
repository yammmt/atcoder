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
    }

    let mut ans = n as i64;
    for i in 0..n {
        let mut curmin = std::i64::MAX;
        for j in i..n {
            curmin = curmin.min(an[j]);
            ans = ans.max(curmin * ((j - i + 1) as i64));
        }
    }

    println!("{}", ans);
}
