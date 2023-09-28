// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n + 1],
        b: [i64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let front = b[i].min(a[i]);
        a[i] -= front;
        ans += front;

        let back = (b[i] - front).min(a[i + 1]);
        a[i + 1] -= back;
        ans += back;
    }
    println!("{}", ans);
}
