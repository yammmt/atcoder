// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    let mut ans = 0.0;
    for i in 1..n + 1 {
        if i >= k {
            ans += 1.0 / n as f64;
            continue;
        }

        let streak = (k as f64 / i as f64).log2().ceil();
        ans += (1.0 / n as f64) * 0.5_f64.powf(streak);
    }
    println!("{}", ans);
}
