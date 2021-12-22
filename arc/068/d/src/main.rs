// 10min -> 7.5min

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut included = HashSet::new();
    an.iter().for_each(|a| { included.insert(*a); });
    let removed = n - included.len();
    println!(
        "{}",
        if removed % 2 == 0 {
            n - removed
        } else {
            n - removed - 1
        }
    );
}
