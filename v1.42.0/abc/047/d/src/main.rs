// -*- coding:utf-8-unix -*-

// 43min
// `an` は相異なるというヒント

use proconio::input;

fn main() {
    input! {
        n: usize,
        _t: usize,
        an: [i64; n],
    }

    let mut cheap = vec![std::i64::MAX; n];
    for i in 1..n {
        cheap[i] = cheap[i - 1].min(an[i - 1]);
    }
    let mut expensive = vec![std::i64::MIN; n];
    for i in (0..n).rev() {
        if i < n - 1 {
            expensive[i] = expensive[i + 1];
        }
        expensive[i] = expensive[i].max(an[i]);
    }
    // println!("{:?}", cheap);
    // println!("{:?}", expensive);

    let mut max_profit = 0;
    for i in 1..n {
        max_profit = max_profit.max(expensive[i] - cheap[i]);
    }

    let mut vans = vec![];
    for i in 1..n {
        if expensive[i] - cheap[i] == max_profit {
            vans.push((cheap[i], expensive[i]));
        }
    }
    vans.dedup();

    println!("{}", vans.len());
}
