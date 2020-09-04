// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;
use std::collections::vec_deque::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        dh: usize,
        dw: usize,
        c: [Chars; h],
    }
    let mut worp_num = vec![vec![std::u64::MAX; w]; h];
    let mut deque = VecDeque::new();
    worp_num[ch - 1][cw - 1] = 0;
    deque.push_back((ch - 1, cw - 1));
    while let Some(base_pt) = deque.pop_front() {
        // println!("base: {:?}", base_pt);
        for i in -1..2 {
            let h_updated = base_pt.0 as isize + i;
            if h_updated < 0 || h_updated >= h as isize {
                continue;
            }

            if c[h_updated as usize][base_pt.1] == '#' {
                continue;
            }

            if worp_num[base_pt.0][base_pt.1] < worp_num[h_updated as usize][base_pt.1] {
                worp_num[h_updated as usize][base_pt.1] = worp_num[base_pt.0][base_pt.1];
                deque.push_front((h_updated as usize, base_pt.1));
            }
        }
        for i in -1..2 {
            let w_updated = base_pt.1 as isize + i;
            if w_updated < 0 || w_updated >= w as isize {
                continue;
            }

            if c[base_pt.0][w_updated as usize] == '#' {
                continue;
            }

            if worp_num[base_pt.0][base_pt.1] < worp_num[base_pt.0][w_updated as usize] {
                worp_num[base_pt.0][w_updated as usize] = worp_num[base_pt.0][base_pt.1];
                deque.push_front((base_pt.0, w_updated as usize));
            }
        }

        for i in -2..3 {
            let h_updated = base_pt.0 as isize + i;
            if h_updated < 0 || h_updated >= h as isize {
                continue;
            }

            for j in -2..3 {
                let w_updated = base_pt.1 as isize + j;
                if w_updated < 0 || w_updated >= w as isize {
                    continue;
                }

                if c[h_updated as usize][w_updated as usize] == '#' {
                    continue;
                }

                if worp_num[base_pt.0][base_pt.1] + 1 < worp_num[h_updated as usize][w_updated as usize] {
                    worp_num[h_updated as usize][w_updated as usize] = worp_num[base_pt.0][base_pt.1] + 1;
                    deque.push_back((h_updated as usize, w_updated as usize));
                }
            }
        }
    }

    if worp_num[dh - 1][dw - 1] == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", worp_num[dh - 1][dw - 1]);
    }
}
