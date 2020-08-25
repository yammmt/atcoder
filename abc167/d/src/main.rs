// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let mut hm = HashMap::new();
    let mut current_town = 0;
    let mut offset = 0;
    let mut cycle = 0;
    hm.insert(current_town, 0);
    // println!("{:?}", a);
    for i in 1..n {
        // print!("{} ", current_town);
        current_town = a[current_town as usize] - 1;
        if i as u64 == k {
            println!("{}", current_town + 1);
            return;
        }

        // println!("to {}", current_town);
        if hm.contains_key(&current_town) == true {
            // println!("{:?}", hm);
            cycle = (i as u64 - hm.get(&current_town).unwrap()) as u64;
            offset = i as u64 - cycle;
            break;
        }

        hm.insert(current_town, i as u64);
    }

    // println!("offset: {}, cycle: {}", offset, cycle);
    let p = (k - offset) % cycle;
    for (key, value) in &hm {
        if *value == p + offset {
            println!("{}", key + 1);
            return;
        }
    }
}
