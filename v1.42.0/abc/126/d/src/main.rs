// -*- coding:utf-8-unix -*-

// 20min 1WA(19.5min)
// WA: デバッグ出力消し忘れ

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        uvwn: [(usize, usize, i64); n - 1],
    }
    let mut vedge = vec![vec![]; n];
    for uvw in &uvwn {
        vedge[uvw.0 - 1].push((uvw.1 - 1, uvw.2));
        vedge[uvw.1 - 1].push((uvw.0 - 1, uvw.2));
    }
    // println!("{:?}", vedge);

    let mut vcolor = vec![std::usize::MAX; n];
    let mut vdq = VecDeque::new();
    vdq.push_back(0);
    vcolor[0] = 0;
    while vcolor.iter().any(|a| *a == std::usize::MAX) {
        let cur = vdq.pop_front().unwrap();
        for (k, v) in &vedge[cur] {
            if vcolor[*k as usize] != std::usize::MAX {
                continue;
            }

            vcolor[*k as usize] = if v % 2 == 0 {
                vcolor[cur]
            } else {
                if vcolor[cur] == 0 {
                    1
                } else {
                    0
                }
            };
            vdq.push_back(*k as usize);
        }
    }

    for c in &vcolor {
        println!("{}", c);
    }
}
