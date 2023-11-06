use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }

    // BFS
    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == '.' || visited[i][j] {
                continue;
            }

            ans += 1;
            let mut que = VecDeque::new();
            que.push_back((i, j));
            while let Some(cur) = que.pop_front() {
                if visited[cur.0][cur.1] {
                    continue;
                }

                visited[cur.0][cur.1] = true;
                for di in -1..=1 {
                    let i_nxt = cur.0.wrapping_add_signed(di);
                    if i_nxt >= h {
                        continue;
                    }

                    for dj in -1..=1 {
                        let j_nxt = cur.1.wrapping_add_signed(dj);
                        if j_nxt >= w {
                            continue;
                        }

                        if !visited[i_nxt][j_nxt] && shw[i_nxt][j_nxt] == '#' {
                            que.push_back((i_nxt, j_nxt));
                        }
                    }
                }
            }
        }
    }

    println!("{ans}");
}
