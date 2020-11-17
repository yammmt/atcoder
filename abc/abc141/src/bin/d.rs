use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [u64; n],
    }

    let mut bh = BinaryHeap::new();
    for a in &an {
        bh.push(*a);
    }

    for _ in 0..m {
        let cur = bh.pop().unwrap();
        bh.push(cur / 2);
    }

    let mut ans = 0;
    while !bh.is_empty() {
        ans += bh.pop().unwrap();
    }
    println!("{}", ans);
}
