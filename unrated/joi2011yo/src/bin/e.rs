// 32.5min
// マスの読み替えと i/j 取り違えで手間取った

// マスの訪問状態はチーズを食べるたびにリセットする
// 直前のマスに戻らない、程度の制限だと無限ループに入る

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mass: [Chars; h],
    }

    let mut new_mass = vec![vec![0; w]; h];
    let mut start_p = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if mass[i][j] == 'S' {
                start_p = (i, j);
            } else if mass[i][j] as u8 >= b'1' && mass[i][j] as u8 <= b'9' {
                new_mass[i][j] = mass[i][j].to_digit(10).unwrap();
            }
        }
    }
    // println!("{:?}", new_mass);

    let mut is_visited_base = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if mass[i][j] == 'X' {
                is_visited_base[i][j] = true;
            }
        }
    }

    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut ans = 0;
    for cheese in 1..n + 1 {
        let mut vdq = VecDeque::new();
        let mut is_visited = is_visited_base.clone();
        vdq.push_back((start_p, 0));
        is_visited[start_p.0][start_p.1] = true;
        'bfs: while let Some(cur) = vdq.pop_front() {
            for d in &dir {
                let inext_i = (cur.0).0 as isize + d.0;
                let inext_j = (cur.0).1 as isize + d.1;
                if inext_i < 0 || inext_j < 0 {
                    continue;
                }

                let unext_i = inext_i as usize;
                let unext_j = inext_j as usize;
                if unext_i < h && unext_j < w && !is_visited[unext_i][unext_j] {
                    if new_mass[unext_i][unext_j] == cheese as u32 {
                        ans += cur.1 + 1;
                        start_p = (unext_i, unext_j);
                        break 'bfs;
                    }
                    is_visited[unext_i][unext_j] = true;
                    vdq.push_back(((unext_i, unext_j), cur.1 + 1));
                }
            }
        }
    }

    println!("{}", ans);
}
