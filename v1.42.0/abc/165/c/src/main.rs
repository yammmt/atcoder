// -*- coding:utf-8-unix -*-

// 58min
// WA: 配列長を n/m 取り違えた + +1 忘れ
// 問題文が読めない first AC も 4:40 なのでそういうもの？

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcdq: [(usize, usize, i64, i64); q],
    }

    let mut vdq = VecDeque::new();
    vdq.push_back((vec![], 1));
    let mut vcomb = vec![];

    while let Some(cur) = vdq.pop_front() {
        // println!("cur: {:?}", cur);
        if cur.1 > m || cur.0.len() > n {
            continue;
        }

        for i in 0..n - cur.0.len() + 1 {
            let mut cv = cur.0.clone();
            for _ in 0..i {
                cv.push(cur.1);
            }
            if cv.len() == n {
                vcomb.push(cv);
            } else if cv.len() < n {
                vdq.push_back((cv, cur.1 + 1));
            }
        }
    }
    // println!("{:?}", vcomb);
    // println!("vcomb.len(): {}", vcomb.len());

    let mut ans = 0;
    for an in &vcomb {
        let mut cur = 0;
        for abcd in &abcdq {
            if an[abcd.1 - 1] as i64 - an[abcd.0 - 1] as i64 == abcd.2 {
                cur += abcd.3;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
