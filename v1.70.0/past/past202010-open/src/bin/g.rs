use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        snn: [Chars; n],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 最大 100 マスしかないので雑に全探索ができる

    let mut no_wall_square_num = 0;
    for i in 0..n {
        for j in 0..m {
            if snn[i][j] == '.' {
                no_wall_square_num += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if snn[i][j] == '.' {
                continue;
            }

            // println!("{i} {j}");
            let mut snn_cur = snn.clone();
            snn_cur[i][j] = '.';

            let mut visited = vec![vec![false; m]; n];
            let mut que = VecDeque::new();
            // 初期値どれでも変わらん
            que.push_back((i, j));
            while let Some((i_cur, j_cur)) = que.pop_front() {
                if visited[i_cur][j_cur] {
                    continue;
                }

                visited[i_cur][j_cur] = true;
                for d in &dir {
                    let i_nxt = i_cur.wrapping_add_signed(d.0);
                    let j_nxt = j_cur.wrapping_add_signed(d.1);
                    if i_nxt >= n || j_nxt >= m || snn_cur[i_nxt][j_nxt] == '#' {
                        continue;
                    }

                    que.push_back((i_nxt, j_nxt));
                }
            }

            let mut visited_num = 0;
            for i in 0..n {
                for j in 0..m {
                    if visited[i][j] {
                        visited_num += 1;
                    }
                }
            }
            // println!("  {visited_num}");

            if visited_num - 1 == no_wall_square_num {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
