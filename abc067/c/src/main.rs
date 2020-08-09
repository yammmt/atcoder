// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [i64; n],
    }

    let mut sunuke: i64 = a.iter().sum();
    let mut raccoon: i64 = 0;
    let mut ans = std::i64::MAX;
    for i in 0..n - 1 {
        sunuke -= a[i as usize];
        raccoon += a[i as usize];
        if (sunuke - raccoon).abs() < ans {
            ans = (sunuke - raccoon).abs();
        }
    }
    println!("{}", ans);
}
