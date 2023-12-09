// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
// use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::collections::BTreeSet;
use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// const DUMMY: usize = usize::MAX / 4;
// const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        an: [usize; n],
        bm: [usize; m],
        cdl: [(usize, usize); l],
    }

    // 主菜を固定し副菜を全探索したいが TLE
    // 食べ合わせの都合で単純なソートは通らない, 二分探索に落とせない
    // だめな組み合わせは高々 L 回しか出現しない
    let mut b_w_i = vec![];
    for (i, &b) in bm.iter().enumerate() {
        b_w_i.push((b, i));
    }
    b_w_i.sort_unstable();
    b_w_i.reverse();

    let mut unavailable = HashSet::new();
    for (c, d) in cdl {
        unavailable.insert((c - 1, d - 1));
    }

    let mut ans = 0;
    for (i, &a) in an.iter().enumerate() {
        let mut bj = 0;
        while bj < m && unavailable.contains(&(i, b_w_i[bj].1)) {
            bj += 1;
        }
        if bj < m {
            ans = ans.max(a + b_w_i[bj].0);
            // println!("{a} + {:?}", b_w_i[bj]);
        }
    }

    println!("{ans}");
}
