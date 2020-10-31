// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut hm = HashMap::new();
    for i in &an {
        let cnt = hm.entry(i).or_insert(0);
        *cnt += 1;
    }
    if hm.len() <= k {
        println!("0");
        return;
    }

    let mut tp = vec![];
    for (k, v) in &hm {
        tp.push((v, k));
    }
    tp.sort();
    let mut written = 0;
    for i in 0..tp.len() - k {
        written += tp[i].0;
    }
    println!("{}", written);
}
