// thanks to: https://atcoder.jp/contests/past201912-open/submissions/15662594

// 独立したダイクストラだと例えば次のパターンで [1][2] (0-indexed) 経由時に誤らない？
// xxxxo
// xxooo
// xxoxx
// ooooo
// と思ったが [4][2] 経由で上書きされるので問題なかった

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_path(graph: &[Vec<i64>], start_pt: (usize, usize)) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![std::i64::MAX; graph[0].len()]; graph.len()];
    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut bh = BinaryHeap::new();
    ret[start_pt.0][start_pt.1] = 0;
    bh.push(Reverse((0, start_pt)));

    while let Some(Reverse((cost, point))) = bh.pop() {
        if cost > ret[point.0][point.1] {
            continue;
        }

        ret[point.0][point.1] = cost;
        for d in &dir {
            let next_point_i = (point.0 as isize + d.0, point.1 as isize + d.1);
            if next_point_i.0 < 0
                || next_point_i.0 >= graph.len() as isize
                || next_point_i.1 < 0
                || next_point_i.1 >= graph[0].len() as isize
            {
                continue;
            }

            let next_point_u = (next_point_i.0 as usize, next_point_i.1 as usize);
            let next_cost = cost + graph[next_point_u.0][next_point_u.1];
            if next_cost < ret[next_point_u.0][next_point_u.1] {
                bh.push(Reverse((
                    cost + graph[next_point_u.0][next_point_u.1],
                    next_point_u,
                )));
            }
        }
    }

    ret
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[i64; w]; h],
    }

    let from_left_bottom = shortest_path(&ahw, (h - 1, 0));
    let from_right_bottom = shortest_path(&ahw, (h - 1, w - 1));
    let from_right_top = shortest_path(&ahw, (0, w - 1));

    let mut ans = std::i64::MAX;
    for i in 0..h {
        for j in 0..w {
            ans = ans.min(
                from_left_bottom[i][j] + from_right_bottom[i][j] + from_right_top[i][j]
                    - 2 * ahw[i][j],
            );
        }
    }

    println!("{}", ans);
}
