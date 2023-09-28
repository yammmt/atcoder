// TODO: 実行時間がギリギリ 見直す

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut vdq = VecDeque::new();
    let mut is_reachable = vec![vec![false; w]; h];
    vdq.push_back((r - 1, c - 1));
    is_reachable[r - 1][c - 1] = true;
    while let Some(cur) = vdq.pop_front() {
        for d in &dir {
            let next_i_i = cur.0 as isize + d.0;
            let next_j_i = cur.1 as isize + d.1;
            if next_i_i < 0
                || next_i_i > h as isize - 1
                || next_j_i < 0
                || next_j_i > w as isize - 1
            {
                continue;
            }

            let next_i_u = next_i_i as usize;
            let next_j_u = next_j_i as usize;
            if is_reachable[next_i_u][next_j_u] {
                continue;
            }

            let next_char = shw[next_i_u][next_j_u];
            if next_char == '#'
                || (next_char == '<' && d != &(0, 1))
                || (next_char == '^' && d != &(1, 0))
                || (next_char == '>' && d != &(0, -1))
                || (next_char == 'v' && d != &(-1, 0))
            {
                continue;
            }

            vdq.push_back((next_i_u, next_j_u));
            is_reachable[next_i_u][next_j_u] = true;
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!(
                "{}",
                if shw[i][j] == '#' {
                    '#'
                } else if is_reachable[i][j] {
                    'o'
                } else {
                    'x'
                }
            );
        }
        println!();
    }
}
