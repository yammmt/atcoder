// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let divisor = 10_u64.pow(9) + 7;
    let ans = (1..=n).fold(1, |p, i| (p * i) % divisor);
    println!("{}", ans);
}
