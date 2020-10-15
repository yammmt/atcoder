// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(u32, u32); m],
    }
    let mut hm = HashMap::new();
    for i in &ab {
        let connected = hm.entry(i.0).or_insert(vec![]);
        connected.push(i.1);
        let connected = hm.entry(i.1).or_insert(vec![]);
        connected.push(i.0);
    }

    let mut vq = VecDeque::new();
    let mut decided = 1;
    let mut ans = vec![0; n + 1];
    vq.push_back(1);
    'wloop: loop {
        // println!("vq: {:?}", vq);
        let current_v = vq.pop_front().unwrap();
        // println!("current_v: {}", current_v);
        if let Some(vv) = hm.get(&current_v) {
            // println!("vv: {:?}", vv);
            for v in vv {
                // 最短側から確定していくので代入は一度きり
                if ans[*v as usize] == 0 {
                    ans[*v as usize] = current_v;
                    decided += 1;
                    if decided == n + 1 {
                        break 'wloop;
                    }
                    vq.push_back(*v);
                }
            }
        }
    }

    // 問題文よりすべての部屋を行き来できるので必ず Yes
    println!("Yes");
    for i in &ans[2..ans.len()] {
        println!("{}", i);
    }
}
