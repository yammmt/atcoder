// :fu: 22-02 遅い

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut k = 0;
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '#' {
                k += 1;
            }
        }
    }

    // この問題文の書き方だと全マス占拠を否めないのでは？と思ったが . マスを含めてはならぬ
    // ヘビマスをすべて並べると 16! で TLE
    // 始点固定して動き方を全探索すると最悪で 4^15 > 10^9 通りあがってきて TLE

    // 数が少ないので履歴を保持してやる
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '.' {
                continue;
            }

            let mut vdq = VecDeque::new();
            vdq.push_back(vec![(i, j)]);
            while let Some(cur) = vdq.pop_front() {
                if cur.len() == k {
                    // pass
                    println!("{}", k);
                    for ans in &cur {
                        println!("{} {}", ans.0 + 1, ans.1 + 1);
                    }
                    return;
                }

                let cur_last = *cur.last().unwrap();
                for d in &dir {
                    let next_i_i = cur_last.0 as isize + d.0;
                    let next_j_i = cur_last.1 as isize + d.1;
                    if next_i_i < 0
                        || next_i_i > h as isize - 1
                        || next_j_i < 0
                        || next_j_i > w as isize - 1
                    {
                        continue;
                    }

                    let next_i_u = next_i_i as usize;
                    let next_j_u = next_j_i as usize;
                    if shw[next_i_u][next_j_u] == '.' || cur.contains(&(next_i_u, next_j_u)) {
                        continue;
                    }

                    let mut next_v = cur.clone();
                    next_v.push((next_i_u, next_j_u));
                    vdq.push_back(next_v);
                }
            }
        }
    }
}
