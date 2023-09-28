// 24min

use proconio::input;
use proconio::marker::Chars;
use std::collections::BinaryHeap;

const DUMMY: isize = std::isize::MAX / 3;

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if shw[i][j] == 'S' {
                start = (i, j);
            } else if shw[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    // println!("{:?}", start);
    // println!("{:?}", goal);

    let mut pass = 0;
    let mut fail = t * h * w + 2;
    while fail - pass > 1 {
        // println!("{} {}", pass, fail);
        let mid = (pass + fail) / 2;

        let mut heap = BinaryHeap::new();
        let mut dist = vec![vec![DUMMY; w]; h];
        // (cost, (x, y))
        heap.push((0isize, start));
        while let Some(cur) = heap.pop() {
            // println!("{:?}", cur);
            if dist[(cur.1).0][(cur.1).1] != DUMMY {
                continue;
            }

            dist[(cur.1).0][(cur.1).1] = -cur.0;
            if cur.1 == goal {
                break;
            }

            for d in &dir {
                let next_x_i = (cur.1).0 as isize + d.0;
                let next_y_i = (cur.1).1 as isize + d.1;
                // println!("{} {}", next_x_i, next_y_i);
                if next_x_i < 0 || next_y_i < 0 || next_x_i >= h as isize || next_y_i >= w as isize
                {
                    continue;
                }

                let next_x_u = next_x_i as usize;
                let next_y_u = next_y_i as usize;
                if dist[next_x_u][next_y_u] != DUMMY {
                    continue;
                }

                match shw[next_x_u][next_y_u] {
                    '.' | 'G' => heap.push((cur.0 - 1, (next_x_u, next_y_u))),
                    'S' => {}
                    '#' => heap.push((cur.0 - mid as isize, (next_x_u, next_y_u))),
                    _ => unreachable!(),
                }
            }
        }

        // println!("{:?}", dist);
        // println!("{}", dist[goal.0][goal.1]);
        if dist[goal.0][goal.1] <= t as isize {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
