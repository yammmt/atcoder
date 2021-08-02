// :fu: :fu: :fu: 21-07 本年最大事故 水色の三完をやたら見る戦犯？
// 解説読んでもいまいち
// そもそも Rust で昇順 PQ 操作して加算する時点で盛大に死ぬ

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }

    // 最小値を PQ でまとめて管理すると値の更新が O(N^2) となり TLE
    // 操作 2 単位で配列を作ると O(N^2) となり TLE
    // 捨てる操作が辛い
    // セグ木を貼るだけ？でもこれ D 問題だしそれしかないならば事故回なので捨て
    // 逆から見る？結局最悪時に無理

    let mut pq = BinaryHeap::new();
    let mut op2_sum = 0;
    for _ in 0..q {
        input! {
            n: usize,
        }

        match n {
            1 => {
                input! {
                    x: i64,
                }
                pq.push(Reverse(x - op2_sum));
            }
            2 => {
                input! {
                    x: i64,
                }
                op2_sum += x;
            }
            3 => {
                let ans = pq.pop().unwrap().0;
                println!("{}", ans + op2_sum);
            }
            _ => unreachable!(),
        }
    }
}
