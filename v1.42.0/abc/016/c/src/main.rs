// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut myfriends = vec![vec![]; n + 1];
    for i in &ab {
        myfriends[i.0].push(i.1);
        myfriends[i.1].push(i.0);
    }
    // println!("{:?}", myfriends);

    for i in 1..n + 1 {
        // println!("i: {}", i);
        let mut cans = 0;
        let mut hs = HashSet::new();
        for j in &myfriends[i] {
            for k in &myfriends[*j as usize] {
                if i != *k && !myfriends[i].contains(&k) && !hs.contains(&k) {
                    // print!("{} ", k);
                    cans += 1;
                    hs.insert(k);
                }
            }
        }
        println!("{}", cans);
    }
}
