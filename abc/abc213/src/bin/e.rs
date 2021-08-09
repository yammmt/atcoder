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
    let crash_dir = [-1, 0, 1];

    // 破壊範囲が一マスであれば 0-1 BFS 典型
    // 愚直にマス目の状態を渡すと O(HHWW) となり TLE
    let mut visited = vec![vec![false; w]; h];
    let mut dp = vec![vec![DUMMY; w]; h];
    let mut vdq = VecDeque::new();
    vdq.push_back(((0, 0), 0));
    while let Some(cur) = vdq.pop_front() {
        if visited[(cur.0).0][(cur.0).1] {
            continue;
        }

        // println!("{:?}", cur.0);
        visited[(cur.0).0][(cur.0).1] = true;
        dp[(cur.0).0][(cur.0).1] = cur.1;
        for d in &dir {
            let h_i = (cur.0).0 as isize + d.0;
            let w_i = (cur.0).1 as isize + d.1;
            if h_i < 0 || w_i < 0 || h_i > h as isize - 1 || w_i > w as isize - 1 {
                continue;
            }

            let h_u = h_i as usize;
            let w_u = w_i as usize;
            // 多分冗長
            if visited[h_u][w_u] || cur.1 >= dp[h_u][w_u] {
                continue;
            }

            if shw[h_u][w_u] == '.' {
                vdq.push_front(((h_u, w_u), cur.1));
            } else {
                vdq.push_back(((h_u, w_u), cur.1 + 1));
                for hd in &crash_dir {
                    for wd in &crash_dir {
                        if *hd == 0 && *wd == 0 {
                            continue;
                        }

                        let c_h_i = h_i + *hd;
                        let c_w_i = w_i + *wd;
                        if c_h_i < 0
                            || c_w_i < 0
                            || c_h_i > h as isize - 1
                            || c_w_i > w as isize - 1
                        {
                            continue;
                        }

                        let c_h_u = c_h_i as usize;
                        let c_w_u = c_w_i as usize;
                        if visited[c_h_u][c_w_u] {
                            continue;
                        }

                        vdq.push_back(((c_h_u, c_w_u), cur.1 + 1));
                    }
                }
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[h - 1][w - 1]);
}
