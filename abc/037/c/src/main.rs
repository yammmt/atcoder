// -*- coding:utf-8-unix -*-

// 4min

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [u64; n],
    }

    let mut csum = an.iter().take(k).sum::<u64>();
    let mut ans = csum;
    for i in k..n {
        csum -= an[i - k];
        csum += an[i];
        ans += csum;
    }
    println!("{}", ans);
}
