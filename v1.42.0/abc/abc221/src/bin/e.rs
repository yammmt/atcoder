// Binary Indexed Tree (BIT) は未履修なので後回し

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
        an: [u64; n],
    }
    let d = 998_244_353;

    // 部分列の先頭と末尾を固定し, それが条件を満たすならば
    // 途中の要素数を m として 2^m を答えに加算し続ければ良い
    // が, これを愚直に実装しては O(N^2) となり TLE
    // A_i の種類数は最大 10^5 個であり, 区間和を高速に出せればできそう
    // セグメントツリー？

    let mut ans = 0;

    println!("{}", ans);
}
