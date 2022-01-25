// 35 (21-08) -> 25 (22-01) 実装が下手

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // パンチを一回繰り出すと最大 4 つのマスにスコアが入る
    let mut scores = vec![vec![DUMMY; w]; h];
    let mut vdq = VecDeque::new();
    // ((現在地), パンチ回数)
    vdq.push_back(((0, 0), 0));
    // スコアも先に更新しておく
    scores[0][0] = 0;
    while let Some(cur) = vdq.pop_front() {
        if cur.1 > scores[(cur.0).0][(cur.0).1] {
            continue;
        }

        for d in &dir {
            let nh_i = (cur.0).0 as isize + d.0;
            let nw_i = (cur.0).1 as isize + d.1;
            if nh_i < 0 || nh_i >= h as isize || nw_i < 0 || nw_i >= w as isize {
                continue;
            }

            let nh_u = nh_i as usize;
            let nw_u = nw_i as usize;
            if cur.1 >= scores[nh_u][nw_u] {
                continue;
            }

            if shw[nh_u][nw_u] == '.' {
                vdq.push_front(((nh_u, nw_u), cur.1));
                scores[nh_u][nw_u] = cur.1;
            } else {
                // パンチ時には (-1, -1) と (1, 1) も判定する
                // (-2..=2) で端だけ弾いたほうが良いし, マスの状態によらずパンチした方がもっと楽
                let punch_dir = match d {
                    (-1, 0) => [(-2, -1), (-2, 0), (-2, 1), (-1, -1), (-1, 0), (-1, 1)],
                    (1, 0) => [(2, -1), (2, 0), (2, 1), (1, -1), (1, 0), (1, 1)],
                    (0, -1) => [(-1, -2), (-1, -1), (0, -2), (0, -1), (1, -2), (1, -1)],
                    (0, 1) => [(-1, 2), (-1, 1), (0, 2), (0, 1), (1, 2), (1, 1)],
                    _ => unreachable!(),
                };

                for pd in &punch_dir {
                    let nh_i = (cur.0).0 as isize + pd.0;
                    let nw_i = (cur.0).1 as isize + pd.1;
                    if nh_i < 0 || nh_i >= h as isize || nw_i < 0 || nw_i >= w as isize {
                        continue;
                    }

                    let ns = cur.1 + 1;
                    let nh_u = nh_i as usize;
                    let nw_u = nw_i as usize;
                    if ns >= scores[nh_u][nw_u] {
                        continue;
                    }

                    vdq.push_back(((nh_u, nw_u), ns));
                    scores[nh_u][nw_u] = ns;
                }
            }
        }
    }
    // println!("{:?}", scores);

    println!("{}", scores[h - 1][w - 1]);
}
