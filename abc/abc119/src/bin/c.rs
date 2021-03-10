// メモ化しない脳筋全探索でも通ってしまった

use proconio::input;
use std::collections::VecDeque;

fn cost(a: i64, originals: &[i64]) -> i64 {
    let ret = (10 * (originals.len() - 1)) as i64;
    let cur = originals.iter().sum::<i64>();
    ret + (cur - a).abs()
}

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        ln: [i64; n],
    }

    // 三つのグループに分けてそれぞれで A/B/C を達成させる
    // 一旦延長短縮のみで作って合成に置き換えて二分探索、では連続性がないのでダメそう
    let mut ans = std::i64::MAX / 2;
    let mut vdq = VecDeque::new();
    vdq.push_back(((vec![], vec![], vec![]), 0));
    while let Some(cur) = vdq.pop_front() {
        if cur.1 == n
            && !((cur.0).0.is_empty())
            && !((cur.0).1.is_empty())
            && !((cur.0).2.is_empty())
        {
            // println!("{:?}", cur);
            // println!("{}", cost(a, &(cur.0).0));
            // println!("{}", cost(b, &(cur.0).1));
            // println!("{}", cost(c, &(cur.0).2));
            ans = ans.min(cost(a, &(cur.0).0) + cost(b, &(cur.0).1) + cost(c, &(cur.0).2));
            continue;
        } else if cur.1 == n {
            continue;
        }

        for i in 0..4 {
            let mut va = ((cur.0).0).clone();
            let mut vb = ((cur.0).1).clone();
            let mut vc = ((cur.0).2).clone();
            match i {
                0 => va.push(ln[cur.1]),
                1 => vb.push(ln[cur.1]),
                2 => vc.push(ln[cur.1]),
                _ => {}
            }
            vdq.push_back(((va, vb, vc), cur.1 + 1));
        }
    }

    println!("{}", ans);
}
