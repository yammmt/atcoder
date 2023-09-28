// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: u64,
        s: u64,
        p: u64,
        t: String,
    }

    let vt: Vec<char> = t.chars().collect();
    let mut skipped = HashSet::new();
    let mut ans = 0;
    for i in 0..n {
        if i >= k {
            if vt[i - k] == vt[i]  {
                // println!("i-k: {}, i: {}", i - k, i);
                // println!("{:?}", skipped);
                if skipped.get(&(i - k)) == None {
                    // can't win
                    skipped.insert(i);
                    continue;
                }
            }
        }
        match vt[i] {
            'r' => ans += p,
            's' => ans += r,
            'p' => ans += s,
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
