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
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// const DUMMY: usize = usize::MAX / 4;
// const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: u32,
        s: u32,
        m: u32,
        l: u32,
    }

    let mut ans = u32::MAX;
    for si in 0..(n / 6 + 2) {
        let egg_s = 6 * si;
        for mj in 0..(n / 8 + 2) {
            let egg_m = 8 * mj;
            for lk in 0..(n / 12 + 2) {
                let egg_l = 12 * lk;
                let egg_sum = egg_s + egg_m + egg_l;
                // println!("{lk}: {egg_sum}");
                if egg_sum >= n {
                    ans = ans.min(s * si + m * mj + l * lk);
                }
            }
        }
    }

    println!("{ans}");
}
