// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        m: usize,
        ps: [(usize, String); m],
    }

    let mut num_ac = 0;
    let mut num_wa = 0;
    let mut hm = HashMap::new();
    let mut hs = HashSet::new();
    for i in 0..m {
        if ps[i].1 == "AC" {
            match hs.get(&ps[i].0) {
                Some(_) => {},
                None => {
                    match hm.get(&ps[i].0) {
                        Some(w) => num_wa += w,
                        None => {},
                    };
                    num_ac += 1;
                    hs.insert(ps[i].0);
                }
            }
        } else {
            let counter = hm.entry(ps[i].0).or_insert(0);
            *counter += 1;
        }
    }
    println!("{} {}", num_ac, num_wa);
}
