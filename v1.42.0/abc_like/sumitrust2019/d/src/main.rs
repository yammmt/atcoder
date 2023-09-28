// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut ans = 0u64;
    let mut hsi = HashSet::new();
    for i in 0..n {
        if hsi.len() == 10 {
            break;
        } else if hsi.contains(&vc[i]) {
            continue;
        }

        hsi.insert(&vc[i]);
        // println!("{}", vc[i]);
        let mut hsj = HashSet::new();
        for j in i + 1..n {
            if hsj.len() == 10 {
                break;
            } else if hsj.contains(&vc[j]) {
                continue;
            }

            hsj.insert(&vc[j]);
            // println!("j: {}", vc[j]);
            let mut uniq = HashSet::new();
            for k in j + 1..n {
                if uniq.len() == 10 {
                    break;
                } else if uniq.contains(&vc[k]) {
                    continue;
                }

                uniq.insert(vc[k]);
            }
            // println!("ans += {:?}", uniq);
            ans += uniq.len() as u64;
        }
    }
    println!("{}", ans);
}
