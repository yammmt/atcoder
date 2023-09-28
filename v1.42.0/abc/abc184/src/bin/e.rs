// 23.5min
// 300ms 超えたけどそんなもんらしい TLE 引っかからなくてえらい

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const NOT_YET: isize = std::isize::MAX / 2;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut char_map = vec![vec![]; 26];
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match ahw[i][j] {
                'S' => start = (i, j),
                'G' => goal = (i, j),
                '.' | '#' => {}
                c => char_map[(c as u8 - b'a') as usize].push((i, j)),
            }
        }
    }

    let mut costs = vec![vec![NOT_YET; w]; h];
    let mut vdq = VecDeque::new();
    vdq.push_back(((start.0, start.1), 1));
    costs[start.0][start.1] = 0;
    let mut teleport_used = vec![false; 26];
    while let Some(cur) = vdq.pop_front() {
        // println!("{}", ahw[(cur.0).0][(cur.0).1]);
        match ahw[(cur.0).0][(cur.0).1] {
            'G' => {}
            '#' => unreachable!(),
            c => {
                // println!("{}", c);
                for d in &dirs {
                    let next_h_i = (cur.0).0 as isize + d.0;
                    let next_w_i = (cur.0).1 as isize + d.1;
                    if next_h_i < 0 || next_w_i < 0 || next_h_i >= h as isize || next_w_i >= w as isize {
                        continue;
                    }

                    let next_h_u = next_h_i as usize;
                    let next_w_u = next_w_i as usize;
                    if ahw[next_h_u][next_w_u] == '#' || costs[next_h_u][next_w_u] != NOT_YET {
                        continue;
                    }

                    costs[next_h_u][next_w_u] = cur.1;
                    vdq.push_back(((next_h_u, next_w_u), cur.1 + 1));
                }

                if c != '.' && c != 'S' {
                    let c_us = (c as u8 - b'a') as usize;
                    if teleport_used[c_us] {
                        continue;
                    }

                    teleport_used[c_us] = true;
                    for mass in &char_map[c_us] {
                        if costs[mass.0][mass.1] == NOT_YET {
                            costs[mass.0][mass.1] = cur.1;
                            vdq.push_back(((mass.0, mass.1), cur.1 + 1));
                        }
                    }
                }
            },
        }
    }
    // println!("{:?}", costs);

    println!(
        "{}",
        if costs[goal.0][goal.1] == NOT_YET {
            -1
        } else {
            costs[goal.0][goal.1]
        }
    );
}
