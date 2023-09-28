// -*- coding:utf-8-unix -*-

// 43min

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    let n = n + 1;
    let d = 10u64.pow(9) + 7;

    let mut isum_min = (1..k + 1).fold(0, |a, b| a + b);
    let mut isum_max = (n - k + 1..n + 1).fold(0, |a, b| a + b);
    let mut ans = (isum_max - isum_min + 1) % d;
    // println!("ans: {}", ans);
    for i in k + 1..n + 1 {
        isum_min = (isum_min + i) % d;
        isum_max = (isum_max + (n - i + 1)) % d;

        ans = (ans + isum_max) % d;
        ans = (ans + d - isum_min) % d;
        ans = (ans + 1) % d;
    }

    println!("{}", ans)
}
