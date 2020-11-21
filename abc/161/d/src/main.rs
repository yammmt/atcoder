// -*- coding:utf-8-unix -*-

// 16min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        k: usize,
    }

    let mut vdq = VecDeque::new();
    for i in 1..10 {
        vdq.push_back(i as u64);
    }
    let mut curk = 0;
    loop {
        let lunlun = vdq.pop_front().unwrap();
        curk += 1;
        if curk == k {
            println!("{}", lunlun);
            return;
        }

        let shiri = lunlun % 10;
        if shiri > 0 {
            vdq.push_back(lunlun * 10 + (shiri - 1));
        }
        vdq.push_back(lunlun * 10 + shiri);
        if shiri < 9 {
            vdq.push_back(lunlun * 10 + (shiri + 1))
        }
    }
}
