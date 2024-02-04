// 制限 4s

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        snn: [Chars; n],
    }
    let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut p1_pos = (DUMMY, DUMMY);
    let mut p2_pos = (DUMMY, DUMMY);
    'outer: for i in 0..n {
        for j in 0..n {
            if snn[i][j] == 'P' {
                if p1_pos.0 == DUMMY {
                    p1_pos = (i, j);
                } else {
                    p2_pos = (i, j);
                    break 'outer;
                }
            }
        }
    }

    let nxt_pos = |x: usize, d| {
        let nx = x.wrapping_add_signed(d);
        if nx > n - 1 {
            if x == 0 {
                0
            } else if x == n - 1 {
                n - 1
            } else {
                unreachable!();
            }
        } else {
            nx
        }
    };

    // (解説放送) Set/Map は log がつくぶん遅くなり今回は危険
    // Rust の HashSet は O(1) だったはずだがそれでも定数倍は重い
    let pos_to_visited_id = |p1: (usize, usize), p2: (usize, usize)| {
        let mut ret = 0;
        ret = ret * n + p1.0;
        ret = ret * n + p1.1;
        ret = ret * n + p2.0;
        ret = ret * n + p2.1;
        ret
    };

    let mut visited = vec![false; n.pow(4)];
    let mut que = VecDeque::new();
    // (p1, p2, 移動回数)
    que.push_back((p1_pos, p2_pos, 0));
    while let Some((p1, p2, move_cnt)) = que.pop_front() {
        if visited[pos_to_visited_id(p1, p2)] {
            continue;
        }

        // println!("{:?}, {:?}, {move_cnt}", p1, p2);
        visited[pos_to_visited_id(p1, p2)] = true;
        for d in &dir {
            // println!("  d: {:?}", d);
            let p1_x_nxt = nxt_pos(p1.0, d.0);
            let p1_y_nxt = nxt_pos(p1.1, d.1);
            let p1_pos = if snn[p1_x_nxt][p1_y_nxt] == '#' {
                p1
            } else {
                (p1_x_nxt, p1_y_nxt)
            };

            let p2_x_nxt = nxt_pos(p2.0, d.0);
            let p2_y_nxt = nxt_pos(p2.1, d.1);
            let p2_pos = if snn[p2_x_nxt][p2_y_nxt] == '#' {
                p2
            } else {
                (p2_x_nxt, p2_y_nxt)
            };

            if visited[pos_to_visited_id(p1_pos, p2_pos)] {
                continue;
            }

            if p1_pos == p2_pos {
                println!("{}", move_cnt + 1);
                return;
            }

            que.push_back((p1_pos, p2_pos, move_cnt + 1));
        }
    }

    println!("-1");
}
