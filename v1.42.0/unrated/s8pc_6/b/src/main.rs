// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vab = Vec::with_capacity(n);
    let mut va = Vec::with_capacity(n);
    let mut vb = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        }
        vab.push((a, b));
        va.push(a);
        vb.push(b);
    }

    va.sort();
    vb.sort();
    let entrance = if n % 2 == 0 {
        ((va[n / 2] as f64 + va[n / 2 - 1] as f64) / 2.0).round() as i64
    } else {
        va[n / 2]
    };
    let exit = if n % 2 == 0 {
        ((vb[n / 2] as f64 + vb[n / 2 - 1] as f64) / 2.0).round() as i64
    } else {
        vb[n / 2]
    };
    let mut ans = 0u64;
    for i in 0..n {
        ans += ((vab[i].0 - entrance).abs() + (vab[i].1 - vab[i].0).abs() + (exit - vab[i].1).abs()) as u64;
    }
    println!("{}", ans);
}
