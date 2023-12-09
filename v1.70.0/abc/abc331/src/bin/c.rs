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
        n: usize,
        an: [usize; n],
    }

    let mut an_sorted = an.clone();
    an_sorted.sort_unstable();

    let mut cusum = vec![0; 1_000_002];
    // let mut cusum = vec![0; 6];
    let mut i_cusum = 0;
    for a in an_sorted {
        while i_cusum < a {
            i_cusum += 1;
            cusum[i_cusum] = cusum[i_cusum - 1];
        }

        cusum[i_cusum] += a;
        // println!("i_cusum: {i_cusum}, {}", cusum[i_cusum]);
        // return;
    }
    // println!("{:?}", cusum[0]);
    // println!("{:?}", cusum[1]);

    while i_cusum < cusum.len() - 1 {
        i_cusum += 1;
        cusum[i_cusum] = cusum[i_cusum - 1];
    }
    // println!("{:?}", cusum[0]);
    // println!("{:?}", cusum[1]);

    for (i, a) in an.iter().enumerate() {
        print!("{}", cusum.last().unwrap() - cusum[*a]);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
