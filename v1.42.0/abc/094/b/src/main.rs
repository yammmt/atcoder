// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        am: [usize; m],
    }

    let mut tolls = HashSet::new();
    for a in &am {
        tolls.insert(*a);
    }

    let mut to_zero = 0;
    for i in 1..x + 1 {
        if tolls.contains(&i) {
            to_zero += 1;
        }
    }

    let mut to_n = 0;
    for i in x..n {
        if tolls.contains(&i) {
            to_n += 1;
        }
    }

    println!("{}", to_zero.min(to_n));
}
