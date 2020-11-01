// -*- coding:utf-8-unix -*-

// 想定解でなく実行時間ギリギリ

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
        st: (usize, usize),
    }

    let mut ve = vec![vec![]; n + 1];
    for e in &uv {
        ve[e.0].push(e.1);
    }
    // println!("ve: {:?}", ve);

    let mut cost = vec![std::u64::MAX; n + 1];
    let mut vdq = VecDeque::new();
    cost[st.0] = 0; // s != t
    vdq.push_back((st.0, 0));
    while !vdq.is_empty() {
        // println!("{:?}", vdq);
        let vinfo = vdq.pop_front().unwrap();
        for v1 in &ve[vinfo.0] {
            for v2 in &ve[*v1 as usize] {
                for v3 in &ve[*v2 as usize] {
                    if cost[*v3] == std::u64::MAX {
                        cost[*v3] = vinfo.1 + 1;
                        if *v3 == st.1 {
                            println!("{}", cost[*v3]);
                            return;
                        }

                        vdq.push_back((*v3, cost[*v3]));
                    }
                }
            }
        }
    }
    println!("-1");
}
