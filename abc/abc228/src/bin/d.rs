// :fu: 21-11 問題文がダルい
// Debug ビルドだと低速
// WA: indexed

// 2
// 1 1048576
// 2 1048576

// 5
// 1 1048576
// 1 1048576
// 2 1
// 2 2
// 2 1048576

use proconio::input;
use std::collections::BTreeSet;

const N: usize = 1_048_576;

fn main() {
    input! {
        q: usize,
        txq: [(usize, usize); q],
    }

    let mut an = vec![-1; N];
    let mut m1s = BTreeSet::new();
    (0..N).for_each(|i| {
        m1s.insert(i);
    });

    for tx in &txq {
        match tx.0 {
            1 => {
                let h = tx.1;
                let mut replaced_h = m1s.range(h % N..).next();
                if replaced_h.is_none() {
                    // 頭から探し直す
                    replaced_h = m1s.range(0..).next();
                }
                let hi = *replaced_h.unwrap();
                an[hi] = tx.1 as isize; // mod とらない
                m1s.remove(&hi);
            }
            2 => println!("{}", an[tx.1 % N]),
            _ => unreachable!(),
        }
    }
}
