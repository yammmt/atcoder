// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut v = vec![];
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        v.push((a, b));
    }
    v.sort();
    let mut left = m as u64;
    let mut ans = 0;
    for i in 0..n {
        let boughtnum = v[i].1.min(left) as u64;
        ans += v[i].0 * boughtnum;
        left -= boughtnum;
        if left == 0 {
            break;
        }
    }
    println!("{}", ans);
}
