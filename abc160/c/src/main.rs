// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        k: u32,
        n: u32,
        a: [u32; n],
    }

    let mut vd: Vec<u32> = Vec::with_capacity(n as usize);
    for i in 0.. n - 1 {
        vd.push(a[(i + 1) as usize] - a[i as usize]);
    }
    vd.push(k + a[0] - a[(n - 1) as usize]);
    vd.sort();
    let ans = k - vd.last().unwrap();
    println!("{}", ans);
}
