// -*- coding:utf-8-unix -*-

// https://takeg.hatenadiary.jp/entry/2019/09/21/204427
// https://betrue12.hateblo.jp/entry/2018/12/08/225123

// :fu:
// 同時期の同 Diff 帯と比べて明らかに苦しい

use proconio::input;
use std::cmp::Ordering;

fn solve(n: u64, x: u64, layer_sum: &[u64], p_sum: &[u64]) -> u64 {
    if n == 0 {
        1
    } else if x == 1 {
        0
    } else {
        let mid = layer_sum[(n - 1) as usize] + 2;
        match x.cmp(&mid) {
            Ordering::Less => solve(n - 1, x - 1, &layer_sum, &p_sum),
            Ordering::Equal => p_sum[(n - 1) as usize] + 1,
            Ordering::Greater => {
                p_sum[(n - 1) as usize]
                    + 1
                    + solve(
                        n - 1,
                        x - mid,
                        &layer_sum,
                        &p_sum
                    )
            }
        }
    }
}

fn main() {
    input! {
        n: u64,
        x: u64,
    }

    let mut layer_sum = vec![];
    layer_sum.push(1u64);
    for i in 1..n + 1 {
        layer_sum.push((2 * layer_sum[(i - 1) as usize] + 3) as u64);
    }
    // println!("{:?}", layer_sum);
    let mut p_sum = vec![];
    p_sum.push(1u64);
    for i in 1..n + 1 {
        p_sum.push((2 * p_sum[(i - 1) as usize] + 1) as u64);
    }
    // println!("{:?}", p_sum);
    println!("{}", solve(n, x, &layer_sum, &p_sum));
}
