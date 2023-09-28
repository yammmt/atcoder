// -*- coding:utf-8-unix -*-

// 30.5min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        vn: [i64; n],
    }
    let mut vdq = VecDeque::new();
    vn.iter().for_each(|v| vdq.push_back(*v));

    let mut ans = 0;

    // a 個取って k - a 個まで返す
    // 宝石数 N < 手順数 K なり得るので境界判定が要る
    for a in 0..k + 1 {
        for b in 0..a + 1 {
            let mut vdq_cur = vdq.clone();
            let mut vcur = vec![];

            // 左から b 個取る
            for _ in 0..b {
                if vdq_cur.is_empty() {
                    break;
                }

                vcur.push(vdq_cur.pop_front().unwrap());
            }

            // 右から a - b 個取る
            if a > b {
                for _ in 0..a - b {
                    if vdq_cur.is_empty() {
                        break;
                    }

                    vcur.push(vdq_cur.pop_back().unwrap());
                }
            }

            // k - a 個まで負の数を返す
            vcur.sort_unstable();
            vcur.reverse();
            for _ in 0..k - a {
                if vcur.is_empty() {
                    break;
                }

                if vcur[vcur.len() - 1] < 0 {
                    vcur.pop();
                }
            }

            if vcur.is_empty() {
                // 初期値 0 であり何もしなくて良い
                continue;
            } else {
                ans = ans.max(vcur.iter().sum::<i64>());
            }
        }
    }

    println!("{}", ans);
}
