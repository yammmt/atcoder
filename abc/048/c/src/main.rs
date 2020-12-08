// -*- coding:utf-8-unix -*-

// 12min

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut an: [i64; n],
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        if an[i] + an[i + 1] > x {
            ans += an[i] + an[i + 1] - x;
            an[i + 1] = (x - an[i]).max(0);
        }
        // println!("{}", ans);
    }
    // println!("{:?}", an);
    println!("{}", ans);
}
