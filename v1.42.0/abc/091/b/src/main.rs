// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let mut hm = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }
        let entry = hm.entry(s).or_insert(0);
        *entry += 1;
    }

    input! {
        m: usize,
    }
    for _ in 0.. m {
        input! {
            s: String,
        }
        let entry = hm.entry(s).or_insert(0);
        *entry -= 1;
    }

    let mut ans = 0;
    for (_, v) in &hm {
        ans = ans.max(*v);
    }
    println!("{}", ans);
}
