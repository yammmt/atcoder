// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i32; m],
    }

    if n >= m {
        println!("0");
        return;
    }

    x.sort();
    let mut v = Vec::with_capacity(m - 1);
    for i in 1..m {
        v.push((x[i] - x[i - 1]).abs());
    }
    v.sort();
    println!("{}", v.iter().take(m - n).sum::<i32>());
}
