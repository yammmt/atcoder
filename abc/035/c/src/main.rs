// -*- coding:utf-8-unix -*-

// 35min.

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut rcnt= vec![0; n + 1];
    for i in &lr {
        rcnt[i.0] += 1;
        if i.1 + 1 < n + 1 {
            rcnt[i.1 + 1] -= 1;
        }
    }
    // println!("{:?}", rcnt);

    let mut rsum = 0;
    for i in 1..n + 1 {
        rsum += rcnt[i];
        if rsum % 2 == 0 {
            print!("0");
        } else {
            print!("1");
        }
    }
    println!("");
}
