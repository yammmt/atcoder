// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        if a[i] == (i + 1) as u32 {
            a.swap(i, i + 1);
            ans += 1;
        }
    }
    if a[n - 1] == n as u32 {
        ans += 1;
    }
    println!("{}", ans);
}
