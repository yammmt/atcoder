// 後ろ二問より難しい気がすると思ったら鬱陶しい解き方をしていた

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const UNVISITED: i64 = 999_999_999;

fn main() {
    input! {
        n: usize,
        m: usize,
        anm_old: [Chars; n],
    }
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut start_nm = (0, 0);
    'outer1: for i in 0..n {
        for j in 0..m {
            if anm_old[i][j] == 'S' {
                start_nm = (i, j);
                break 'outer1;
            }
        }
    }

    let mut goal_nm = (0, 0);
    'outer2: for i in 0..n {
        for j in 0..m {
            if anm_old[i][j] == 'G' {
                goal_nm = (i, j);
                break 'outer2;
            }
        }
    }

    // 0 から 10 に行くとする
    let mut anm = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            anm[i][j] = if anm_old[i][j] == 'S' {
                0
            } else if anm_old[i][j] == 'G' {
                10
            } else {
                (anm_old[i][j] as u8 - b'0') as usize
            };
        }
    }

    let mut shortest_pathes = vec![vec![vec![UNVISITED; m]; n]; 10];
    shortest_pathes[0][start_nm.0][start_nm.1] = 0;

    let mut vdq = VecDeque::new();
    // 基点, 今の移動回数, 今のステージ
    vdq.push_back((start_nm, 0, 0));
    while let Some(cur) = vdq.pop_front() {
        for d in &dir {
            let next_n = (cur.0).0 as isize + d.0;
            let next_m = (cur.0).1 as isize + d.1;
            if next_n < 0 || next_m < 0 || next_n as usize >= n || next_m as usize >= m {
                // マス外
                continue;
            }

            let next_n_u = next_n as usize;
            let next_m_u = next_m as usize;
            if cur.1 + 1 >= shortest_pathes[cur.2][next_n_u][next_m_u] {
                // 最短でない
                continue;
            }

            shortest_pathes[cur.2][next_n_u][next_m_u] = cur.1 + 1;
            vdq.push_back(((next_n_u, next_m_u), cur.1 + 1, cur.2));
        }

        if vdq.is_empty() && cur.2 != 9 {
            // 次の数字を探しにゆく
            // 基点となるマスを vdq に入れ直す
            for i in 0..n {
                for j in 0..m {
                    if anm[i][j] == cur.2 + 1 && shortest_pathes[cur.2][i][j] != UNVISITED {
                        vdq.push_back(((i, j), shortest_pathes[cur.2][i][j], cur.2 + 1));
                        shortest_pathes[cur.2 + 1][i][j] = shortest_pathes[cur.2][i][j];
                    }
                }
            }
        }
    }

    println!(
        "{}",
        if shortest_pathes[9][goal_nm.0][goal_nm.1] == UNVISITED {
            -1
        } else {
            shortest_pathes[9][goal_nm.0][goal_nm.1]
        }
    );
}
