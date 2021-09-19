// 条件を読み取る部分が難しい

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        a44: [[usize; 4]; 4],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 2^(4*4) = 65,536 であり保護するかどうか全探索できる
    // すべての家が囲まれているかはすぐに判定できる
    let mut ans = 0;
    for i in 0..2u32.pow(4 * 4) {
        // マス作成
        let mut cur_block = vec![vec![0; 4]; 4];
        let mut cur_i = i;
        for j in 0..16 {
            if cur_i % 2 == 1 {
                cur_block[j / 4][j % 4] = 1;
            }
            cur_i /= 2;
        }

        // すべての家が保護されている？
        let mut house_pass = true;
        'outer: for ii in 0..4 {
            for jj in 0..4 {
                if a44[ii][jj] == 1 && cur_block[ii][jj] == 0 {
                    house_pass = false;
                    break 'outer;
                }
            }
        }
        if !house_pass {
            continue;
        }

        // BFS: 壁の連結性を見る, 連結でなければならないので始点は一点のみ
        let mut que = VecDeque::new();
        let mut visited = vec![vec![false; 4]; 4];
        'qin_outer: for ii in 0..4 {
            for jj in 0..4 {
                if cur_block[ii][jj] == 1 {
                    visited[ii][jj] = true;
                    que.push_front((ii, jj));
                    break 'qin_outer;
                }
            }
        }

        while let Some(cur) = que.pop_front() {
            for d in &dir {
                let next_i_i = cur.0 as isize + d.0;
                let next_j_i = cur.1 as isize + d.1;
                if next_i_i < 0 || next_i_i > 3 || next_j_i < 0 || next_j_i > 3 {
                    continue;
                }

                let next_i_u = next_i_i as usize;
                let next_j_u = next_j_i as usize;
                if !visited[next_i_u][next_j_u] && cur_block[next_i_u][next_j_u] == 1 {
                    visited[next_i_u][next_j_u] = true;
                    que.push_front((next_i_u, next_j_u));
                }
            }
        }
        let mut block_pass = true;
        'guard_check: for ii in 0..4 {
            for jj in 0..4 {
                if cur_block[ii][jj] == 1 && !visited[ii][jj] {
                    block_pass = false;
                    break 'guard_check;
                }
            }
        }
        if !block_pass {
            continue;
        }

        // BFS: 穴が空いた図形は満たさない
        // 保護されない領域を始点に BFS では判定できない
        // すべての 0 から外周を抜けられれば良い？
        let mut hole_pass = true;
        let mut visited = vec![vec![false; 4]; 4];
        'que2_outer: for ii in 0..4 {
            for jj in 0..4 {
                if cur_block[ii][jj] == 0 && !visited[ii][jj] {
                    let mut que = VecDeque::new();
                    que.push_front((ii, jj));
                    visited[ii][jj] = true;
                    let mut reach_goal = false;
                    while let Some(cur) = que.pop_front() {
                        for d in &dir {
                            let next_i_i = cur.0 as isize + d.0;
                            let next_j_i = cur.1 as isize + d.1;
                            if next_i_i < 0 || next_i_i > 3 || next_j_i < 0 || next_j_i > 3 {
                                reach_goal = true;
                                continue;
                            }

                            let next_i_u = next_i_i as usize;
                            let next_j_u = next_j_i as usize;
                            if !visited[next_i_u][next_j_u] && cur_block[next_i_u][next_j_u] == 0 {
                                visited[next_i_u][next_j_u] = true;
                                que.push_front((next_i_u, next_j_u));
                            }
                        }
                    }
                    if !reach_goal {
                        // for b in &cur_block {
                        //     println!("{:?}", b);
                        // }
                        // println!();
                        hole_pass = false;
                        break 'que2_outer;
                    }
                }
            }
        }

        if hole_pass {
            // for b in &cur_block {
            //     println!("{:?}", b);
            // }
            // println!();
            ans += 1;
        }
    }

    println!("{}", ans);
}
