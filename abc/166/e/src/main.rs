// -*- coding:utf-8-unix -*-

// 13min (別解)

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }
    let mut hm: HashMap<i64, i64> = HashMap::new();
    for (i, a) in an.iter().enumerate() {
        let cnt = hm.entry(i as i64 - *a).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0;
    for (i, a) in an.iter().enumerate() {
        match hm.get(&(i as i64 + *a)) {
            Some(n) => ans += n,
            None => {},
        }
    }
    println!("{}", ans);
}
