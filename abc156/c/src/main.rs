// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }

    let sum_p = x.iter().sum::<i64>() as f64;
    let p = (sum_p / n as f64).round() as i64;
    // println!("{} {}", sum_p, p);
    let mut ans = 0;
    for v in &x {
        ans += (*v - p).pow(2);
    }
    println!("{}", ans);
}
