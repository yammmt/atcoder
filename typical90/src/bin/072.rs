// > 30min
// :fu: :fu: :fu: 21-06 bitDP どうするの

use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut ans = -1;
    for i in 0..h {
        for j in 0..w {
            if chw[i][j] == '#' {
                continue;
            }

            let mut vdq = VecDeque::new();
            vdq.push_back(((i, j), HashSet::new()));
            while let Some(mut cur) = vdq.pop_front() {
                cur.1.insert(cur.0);
                for d in &dir {
                    let next_h_i = (cur.0).0 as isize + d.0;
                    let next_w_i = (cur.0).1 as isize + d.1;
                    if next_h_i >= h as isize
                        || next_h_i < 0
                        || next_w_i >= w as isize
                        || next_w_i < 0
                    {
                        continue;
                    }

                    let next_h_u = next_h_i as usize;
                    let next_w_u = next_w_i as usize;
                    if next_h_u == i && next_w_u == j {
                        if cur.1.len() > 2 {
                            ans = ans.max(cur.1.len() as isize);
                        }
                        continue;
                    } else if chw[next_h_u][next_w_u] == '#'
                        || cur.1.contains(&(next_h_u, next_w_u))
                    {
                        continue;
                    }

                    vdq.push_back(((next_h_u, next_w_u), cur.1.clone()));
                }
            }
        }
    }

    println!("{}", ans);
}
