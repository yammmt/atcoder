// WA: 負方向に迂回した場合のマスが足りなかった

use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

const NOT_YET: usize = std::usize::MAX / 2;

fn icor_to_ucor((x, y): (isize, isize)) -> (usize, usize) {
    ((x + 202) as usize, (y + 202) as usize)
}

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        xyn: [(isize, isize); n],
    }
    let dir = [(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];
    let goal = icor_to_ucor((x, y));
    let mut forbidden = HashSet::new();
    for xy in &xyn {
        forbidden.insert(icor_to_ucor((xy.0, xy.1)));
    }

    let mut shortest_paths = vec![vec![NOT_YET; 410]; 410];
    let mut vdq = VecDeque::new();
    vdq.push_back(((202, 202), 1));
    shortest_paths[202][202] = 0;
    'bfs: while let Some(cur) = vdq.pop_front() {
        for d in &dir {
            let next_x_i = (cur.0).0 as isize + d.0;
            let next_y_i = (cur.0).1 as isize + d.1;
            if next_x_i < 0 || next_y_i < 0 || next_x_i > 408 || next_y_i > 408 {
                continue;
            }

            let next_x_u = next_x_i as usize;
            let next_y_u = next_y_i as usize;
            if shortest_paths[next_x_u][next_y_u] != NOT_YET || forbidden.contains(&(next_x_u, next_y_u)) {
                continue;
            }

            shortest_paths[next_x_u][next_y_u] = cur.1;
            vdq.push_back(((next_x_u, next_y_u), cur.1 + 1));

            if next_x_u == goal.0 && next_y_u == goal.1 {
                break 'bfs;
            }
        }
    }

    println!(
        "{}",
        if shortest_paths[goal.0][goal.1] == NOT_YET {
            -1
        } else {
            shortest_paths[goal.0][goal.1] as isize
        }
    );
}
