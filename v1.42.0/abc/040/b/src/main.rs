// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = n - 1;
    // 単純な O(n^2) でも大差なし
    let i_max = (n as f64).sqrt().floor() as i64;
    for i in 1..i_max + 1 {
        for j in i..n {
            if i * j > n {
                break;
            }

            ans = ans.min(j - i + (n - i * j));
        }
    }
    println!("{}", ans);
}
