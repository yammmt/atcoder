// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        v.push((a, b));
    }
    v.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:?}", v);

    let mut total_time = 0;
    for ab in &v {
        total_time += ab.0;
        if total_time > ab.1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
