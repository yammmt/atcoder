// 0-1 BFS

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const DUMMY: usize = std::usize::MAX / 3;

fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut start = (0, 0);
    'outer: for i in 0..h {
        for j in 0..w {
            if chw[i][j] == 's' {
                start = (i, j);
                break 'outer;
            }
        }
    }
    let start = start;

    let mut vdq = VecDeque::new();
    let mut cost = vec![vec![DUMMY; w]; h];
    // ((point), cost)
    vdq.push_back((start, 0));
    while let Some(cur) = vdq.pop_front() {
        if cost[(cur.0).0][(cur.0).1] != DUMMY {
            continue;
        }

        cost[(cur.0).0][(cur.0).1] = cur.1;
        for d in &dir {
            let next_i = ((cur.0).0 as isize + d.0, (cur.0).1 as isize + d.1);
            if next_i.0 < 0 || next_i.1 < 0 || next_i.0 >= h as isize || next_i.1 >= w as isize {
                continue;
            }

            let next_u = (next_i.0 as usize, next_i.1 as usize);
            if cost[next_u.0][next_u.1] != DUMMY || (cur.1 == 2 && chw[next_u.0][next_u.1] == '#')
            {
                continue;
            }

            match chw[next_u.0][next_u.1] {
                's' | '.' => vdq.push_front((next_u, cur.1)),
                'g' => {
                    println!("YES");
                    return;
                }
                '#' => vdq.push_back((next_u, cur.1 + 1)),
                _ => unreachable!(),
            }
        }
    }

    println!("NO");
}
