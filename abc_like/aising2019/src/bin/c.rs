// 35min 2WA
// WA1 (21min): 400^3 なら TLE 回避できると思った (実際は 400^4)
// WA2 (28min): 400x400 なら 32bit 範囲内だと思った

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        sh: [Chars; h],
    }
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    // 0: 未発見
    let mut grp = vec![vec![0; w]; h];
    let mut grp_num = 1;
    for i in 0..h {
        for j in 0..w {
            if grp[i][j] != 0 {
                continue;
            }

            grp[i][j] = grp_num;
            let mut vdq = VecDeque::new();
            vdq.push_back((i, j));
            while let Some(cur) = vdq.pop_front() {
                for d in &dir {
                    let next_i = cur.0 as isize + d.0;
                    let next_j = cur.1 as isize + d.1;
                    if next_i < 0 || next_j < 0 || next_i >= h as isize || next_j >= w as isize {
                        continue;
                    }

                    let next_i_u = next_i as usize;
                    let next_j_u = next_j as usize;
                    if grp[next_i_u][next_j_u] != 0 || sh[cur.0][cur.1] == sh[next_i_u][next_j_u] {
                        continue;
                    }

                    grp[next_i_u][next_j_u] = grp_num;
                    vdq.push_back((next_i_u, next_j_u));
                }
            }
            grp_num += 1;
        }
    }
    // println!("{:?}", grp);

    let mut grp_len = vec![(0, 0); h * w + 1];
    for i in 0..h {
        for j in 0..w {
            if sh[i][j] == '#' {
                grp_len[grp[i][j]].0 += 1;
            } else {
                grp_len[grp[i][j]].1 += 1;
            }
        }
    }
    let mut ans = 0i64;
    grp_len.iter().skip(1).for_each(|g| ans += g.0 * g.1);
    println!("{}", ans);
}
