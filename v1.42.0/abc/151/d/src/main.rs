// -*- coding:utf-8-unix -*-

// 16min

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for start_h in 0..h {
        for start_w in 0..w {
            if s[start_h][start_w] == '#' {
                continue;
            }

            let mut vdq = VecDeque::new();
            let mut visited = vec![vec![false; w]; h];
            visited[start_h][start_w] = true;
            vdq.push_back(((start_h, start_w), 0));
            while !vdq.is_empty() {
                let nowp = vdq.pop_front().unwrap();
                ans = ans.max(nowp.1);

                if (nowp.0).0 > 0 && s[(nowp.0).0 - 1][(nowp.0).1] == '.' && !visited[(nowp.0).0 - 1][(nowp.0).1] {
                    vdq.push_back((((nowp.0).0 - 1, (nowp.0).1), nowp.1 + 1));
                    visited[(nowp.0).0 - 1][(nowp.0).1] = true;
                }
                if (nowp.0).0 < h - 1 && s[(nowp.0).0 + 1][(nowp.0).1] == '.' && !visited[(nowp.0).0 + 1][(nowp.0).1] {
                    vdq.push_back((((nowp.0).0 + 1, (nowp.0).1), nowp.1 + 1));
                    visited[(nowp.0).0 + 1][(nowp.0).1] = true;
                }
                if (nowp.0).1 > 0 && s[(nowp.0).0][(nowp.0).1 - 1] == '.' && !visited[(nowp.0).0][(nowp.0).1 - 1] {
                    vdq.push_back((((nowp.0).0, (nowp.0).1 - 1), nowp.1 + 1));
                    visited[(nowp.0).0][(nowp.0).1 - 1] = true;
                }
                if (nowp.0).1 < w - 1 && s[(nowp.0).0][(nowp.0).1 + 1] == '.' && !visited[(nowp.0).0][(nowp.0).1 + 1] {
                    vdq.push_back((((nowp.0).0, (nowp.0).1 + 1), nowp.1 + 1));
                    visited[(nowp.0).0][(nowp.0).1 + 1] = true;
                }
            }
        }
    }
    println!("{}", ans);
}
