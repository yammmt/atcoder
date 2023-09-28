// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let mut hm = HashMap::new();
    for s in &sn {
        let counter = hm.entry(s).or_insert(0);
        *counter += 1;
    }

    let mut ans = vec![];
    let mut maxn = 0;
    for (k, v) in &hm {
        if *v > maxn {
            maxn = *v;
            ans = vec![k];
        } else if *v == maxn {
            ans.push(k);
        }
    }
    ans.sort();
    for s in &ans {
        println!("{}", s);
    }
}
