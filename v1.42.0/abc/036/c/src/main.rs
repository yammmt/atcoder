// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vai = vec![];
    for i in 0..n {
        input! {
            a: usize,
        }
        vai.push((a, i));
    }

    vai.sort();
    let mut prize = 0;
    let mut vans = vec![0; n];
    for i in 0..n {
        if i > 0 && vai[i].0 > vai[i - 1].0 {
            prize += 1;
        }
        vans[vai[i].1] = prize;
    }
    for i in &vans {
        println!("{}", i);
    }
}
