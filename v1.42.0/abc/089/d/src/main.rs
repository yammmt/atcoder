// -*- coding:utf-8-unix -*-

use proconio::input;

fn move_cost(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        ahw: [[usize; w]; h],
        q: usize,
        lrq: [(usize, usize); q],
    }

    let mut apos = vec![(0, 0); h * w + 1];
    for i in 0..h {
        for j in 0..w {
            apos[ahw[i][j] as usize] = (i as i64, j as i64);
        }
    }

    // 移動幅が d の倍数で固定されているため特定のマスへの遷移は一通りに定まる
    let mut ans = vec![0; h * w + 1];
    for i in 0..ans.len() {
        if i < d {
            continue;
        }

        ans[i] = ans[i - d] + move_cost(apos[i], apos[i - d]);
    }

    for lr in &lrq {
        println!("{}", ans[lr.1] - ans[lr.0]);
    }
}
