// :fu: :fu: 21-05 発想はともかく重実装
// 計算間に合う？ 1000 x 1000 x 4?
// WA: 0-1BFS 初回目的地到着時に探索を終えたらダメだったがどうして？

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut rcs: (usize, usize),
        mut rct: (usize, usize),
        shw: [Chars; h],
    }
    let rcs = (rcs.0 - 1, rcs.1 - 1);
    let rct = (rct.0 - 1, rct.1 - 1);
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut vdq = VecDeque::new();
    let mut min_num = vec![vec![vec![std::usize::MAX / 2; 4]; w]; h];
    for i in 0..4 {
        min_num[rcs.0][rcs.1][i] = 0;
    }
    for (i, d) in dir.iter().enumerate() {
        let h_i = rcs.0 as isize + d.0;
        let w_i = rcs.1 as isize + d.1;
        if h_i < 0 || w_i < 0 || h_i >= h as isize || w_i >= w as isize {
            continue;
        }

        let h_u = h_i as usize;
        let w_u = w_i as usize;
        if shw[h_u][w_u] == '.' {
            // (座標), 進行方向, 移動方向変更数
            vdq.push_back(((h_u, w_u), i, 0));
        }
    }

    while let Some(cur) = vdq.pop_front() {
        if cur.2 >= min_num[(cur.0).0][(cur.0).1][cur.1] {
            continue;
        }

        min_num[(cur.0).0][(cur.0).1][cur.1] = cur.2;

        for (i, d) in dir.iter().enumerate() {
            let h_i = (cur.0).0 as isize + d.0;
            let w_i = (cur.0).1 as isize + d.1;
            if h_i < 0 || w_i < 0 || h_i >= h as isize || w_i >= w as isize {
                continue;
            }

            let h_u = h_i as usize;
            let w_u = w_i as usize;
            let next_cost = if i == cur.1 { cur.2 } else { cur.2 + 1 };
            if shw[h_u][w_u] == '.' && next_cost < min_num[h_u][w_u][i] {
                // (座標), 進行方向, 移動方向変更数
                if i == cur.1 {
                    vdq.push_front(((h_u, w_u), i, next_cost));
                } else {
                    vdq.push_back(((h_u, w_u), i, next_cost));
                }
            }
        }
    }

    println!("{}", min_num[rct.0][rct.1].iter().min().unwrap());
}
