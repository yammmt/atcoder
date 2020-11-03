// -*- coding:utf-8-unix -*-

// いもす法使おうとしてサイズ max(xh.0) (= 最大で 10^9 個分) のベクトル用意すると RE になる
// https://atcoder.jp/contests/abc153/submissions/17857367
// がメモリ使用量には余裕がある

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: u64,
        mut xh: [(usize, u64); n],
    }
    xh.sort();

    let mut ans = 0u64;
    let mut vdq = VecDeque::<(usize, u64)>::new();
    let mut cdmg = 0u64;
    for i in &xh {
        while !vdq.is_empty() {
            let recovery = vdq.pop_front().unwrap();
            if recovery.0 <= i.0 {
                cdmg -= recovery.1;
            } else {
                vdq.push_front(recovery);
                break;
            }
        }

        if cdmg as u64 >= i.1 {
            // もう死んでいるので何もしない
            continue;
        }

        let rest = i.1 - cdmg;
        let mut atk = (rest / a) * a;
        ans += rest / a;
        if rest % a != 0 {
            atk += a;
            ans += 1;
        }
        cdmg += atk;
        let dmgout = i.0 + 2 * d + 1;
        if dmgout <= xh[n - 1].0 {
            vdq.push_back((dmgout, atk));
        }
    }
    println!("{}", ans);
}
