// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    a.sort();
    a.reverse();
    let mut ans = a.iter().take(n / 2).map(|&a| 2 * a).sum::<u64>();
    ans -= a[0];
    if n % 2 == 1 {
        ans += a[n / 2];
    }
    println!("{}", ans);
}
