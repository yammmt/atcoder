// -*- coding:utf-8-unix -*-

// 33.5min

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }
    let d = 10u64.pow(9) + 7;
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut ahw_order = vec![];
    for i in 0..h {
        for j in 0..w {
            ahw_order.push((ahw[i][j], (i, j)));
        }
    }
    ahw_order.sort_unstable();

    let mut ans = 0;
    let mut ways = vec![vec![1u64; w]; h];
    for cur in ahw_order {
        ans = (ans + ways[(cur.1).0][(cur.1).1]) % d;

        for d_n in &dir {
            let next_i = (cur.1).0 as isize + d_n.0;
            let next_j = (cur.1).1 as isize + d_n.1;
            if next_i < 0 || next_j < 0 || next_i >= h as isize || next_j >= w as isize {
                continue;
            }

            let next_i = next_i as usize;
            let next_j = next_j as usize;
            if ahw[next_i][next_j] > ahw[(cur.1).0][(cur.1).1] {
                ways[next_i][next_j] = (ways[next_i][next_j] + ways[(cur.1).0][(cur.1).1]) % d;
            }
        }
    }
    // println!("{:?}", ways);

    println!("{}", ans);
}
