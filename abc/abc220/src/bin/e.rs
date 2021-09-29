// LCA 未履修

// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let dd = 998_244_353;

    // F と同様に一頂点分動くとすべての点に対して記憶している距離が +1/-1 のどちらかだけずれる
    // 頂点 1 起点に全頂点への距離を持って今いくつ動いたのかをうまく管理すればできそうなのだが
    // 愚直に距離をベクトルにもたせて BFS すると明らかに TLE
}
