// 14.5min (2020-11) -> 10min(2022-1)

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const DUMMY: usize = std::usize::MAX / 2;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 通った白マスの数 (既に踏んだ白マスの数)
    let mut scores = vec![vec![DUMMY; w]; h];
    let mut vdq = VecDeque::new();
    vdq.push_back((0, 0));
    scores[0][0] = 1;
    while let Some(cur) = vdq.pop_front() {
        let nscore = scores[cur.0][cur.1] + 1;
        for d in &dir {
            let nh_i = cur.0 as isize + d.0;
            let nw_i = cur.1 as isize + d.1;
            if nh_i < 0 || nh_i >= h as isize || nw_i < 0 || nw_i >= w as isize {
                continue;
            }

            let nh_u = nh_i as usize;
            let nw_u = nw_i as usize;
            if nscore < scores[nh_u][nw_u] && shw[nh_u][nw_u] == '.' {
                vdq.push_back((nh_u, nw_u));
                scores[nh_u][nw_u] = nscore;
            }
        }
    }

    if scores[h - 1][w - 1] == DUMMY {
        println!("-1");
        return;
    }

    let white_num = shw.iter().fold(0, |su, row| {
        su + row.iter().filter(|&&c| c == '.').count()
    });

    println!("{}", white_num - scores[h - 1][w - 1]);
}
