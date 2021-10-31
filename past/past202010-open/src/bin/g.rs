use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        sn: [Chars; n],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut ans = 0;
    // 愚直に全マスで試そう
    for i in 0..n {
        for j in 0..m {
            if sn[i][j] == '.' {
                continue;
            }

            let mut cur_map = sn.clone();
            cur_map[i][j] = '#';
            let mut visited = vec![vec![false; m]; n];
            let mut vdq = VecDeque::new();
            vdq.push_back((i, j));
            visited[i][j] = true;
            while let Some(cur) = vdq.pop_front() {
                for d in &dir {
                    let next_i_i = cur.0 as isize + d.0;
                    if next_i_i < 0 || next_i_i > n as isize - 1 {
                        continue;
                    }

                    let next_i_u = next_i_i as usize;

                    let next_j_i = cur.1 as isize + d.1;
                    if next_j_i < 0 || next_j_i > m as isize - 1 {
                        continue;
                    }

                    let next_j_u = next_j_i as usize;

                    if !visited[next_i_u][next_j_u] && cur_map[next_i_u][next_j_u] == '.' {
                        vdq.push_back((next_i_u, next_j_u));
                        visited[next_i_u][next_j_u] = true;
                    }
                }
            }
            // println!("{} {}", i, j);
            // println!("{:?}",   cur_map);
            // println!("{:?}",   visited);

            let mut pass = true;
            'outer: for ii in 0..n {
                for jj in 0..m {
                    if cur_map[ii][jj] == '.' && !visited[ii][jj] {
                        pass = false;
                        break 'outer;
                    }
                }
            }

            if pass {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
