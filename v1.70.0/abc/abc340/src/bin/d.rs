use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        abxn: [(usize, usize, Usize1); n - 1],
    }

    let mut costs = vec![DUMMY; n];
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), 0));
    while let Some((Reverse(cost_cur), v_cur)) = bh.pop() {
        if v_cur == n - 1 {
            println!("{cost_cur}");
            return;
        }

        if costs[v_cur] != DUMMY {
            continue;
        }

        costs[v_cur] = cost_cur;
        // 丁寧に重複省いてもいいけど省かなくても間に合う
        bh.push((Reverse(cost_cur + abxn[v_cur].0), v_cur + 1));
        bh.push((Reverse(cost_cur + abxn[v_cur].1), abxn[v_cur].2));
    }
}
