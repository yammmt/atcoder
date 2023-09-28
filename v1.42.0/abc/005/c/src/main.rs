// -*- coding:utf-8-unix -*-

// 17min.

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        t: i32,
        n: usize,
        an: [i32; n],
        m: usize,
        bm: [i32; m],
    }

    let mut vdq = VecDeque::new();
    let mut aidx = 0;
    let mut bidx = 0;
    for i in 0..bm[m - 1] + 1 {
        // たこ追加
        while aidx < n && an[aidx] == i {
            if an[aidx] == i {
                vdq.push_back(i + t);
                aidx += 1;
            }
        }

        // たこ消費
        while bidx < m && bm[bidx] == i {
            if bm[bidx] == i {
                let mut is_sold = false;
                while !is_sold {
                    if vdq.is_empty() {
                        break;
                    }

                    let now = vdq.pop_front().unwrap();
                    if now >= bm[bidx] {
                        is_sold = true;
                    }
                }
                if !is_sold {
                    println!("no");
                    return;
                }
            }
            bidx += 1;
        }
    }
    println!("yes");
}
