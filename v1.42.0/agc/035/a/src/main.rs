// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut hm = HashMap::new();
    for i in &a {
        let entry = hm.entry(i).or_insert(0);
        *entry += 1;
    }

    match hm.len() {
        3 => {
            if n % 3 != 0 {
                println!("No");
                return;
            }
            let mut xorsum = 0;
            for (&k, &v) in &hm {
                if v != n / 3 {
                    println!("No");
                    return;
                }
                xorsum ^= k;
            }
            if xorsum == 0x00 {
                println!("Yes");
            } else {
                println!("No");
            }
        },
        2 => {
            if n % 3 != 0 || hm.get(&0) == None {
                println!("No");
                return;
            }
            if hm[&0] != n / 3 {
                println!("No");
                return;
            }
            println!("Yes");
        },
        1 => {
            match a[0] {
                0 => println!("Yes"),
                _ => println!("No"),
            }
        }
        _ => println!("No"),
    }
}
