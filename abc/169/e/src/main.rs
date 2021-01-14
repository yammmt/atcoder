// -*- coding:utf-8-unix -*-

// :fu:
// 数問

// > 70min, ほぼすべて偶数時の算数

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut lower = abn.iter().map(|ab| ab.0).collect::<Vec<i64>>();
    let mut upper = abn.iter().map(|ab| ab.1).collect::<Vec<i64>>();
    lower.sort_unstable();
    upper.sort_unstable();

    println!(
        "{}",
        if n % 2 == 0 {
            // :fu:
            // 最大値最小値共に分母が 2 であるため、分子の動く範囲だけを考えれば良い
            (upper[n / 2] + upper[n / 2 - 1] - (lower[n / 2] + lower[n / 2 - 1])) + 1
        } else {
            upper[n / 2] - lower[n / 2] + 1
        }
    );
}
