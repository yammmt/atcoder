// FIXME: HashSet で重複探索を消しても "a...a" のような単調ケースにしか通らない

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t3: Chars,
    }

    // 頭から貪欲では WA (例 2)
    // N が小さいので消す位置を全探索し続ければ良い？
    // N 通りの消し方それぞれに対し N 回の新文字列作成で無事 TLE

    let mut hs = HashSet::new();
    let mut ans = 0;
    let mut vdq = VecDeque::new();
    // (スコア, 文字列)
    vdq.push_back((0, s.clone()));
    hs.insert(s);
    while let Some(cur) = vdq.pop_front() {
        for i in 2..cur.1.len() {
            if cur.1[i - 2] == t3[0] && cur.1[i - 1] == t3[1] && cur.1[i] == t3[2] {
                let mut nv = vec![];
                for j in 0..cur.1.len() {
                    if j < i - 2 || j > i {
                        nv.push(cur.1[j]);
                    }
                }

                if hs.contains(&nv) {
                    continue;
                }

                let score = cur.0 + 1;
                ans = ans.max(score);
                vdq.push_back((score, nv.clone()));
                hs.insert(nv);
            }
        }
    }

    println!("{}", ans);
}
