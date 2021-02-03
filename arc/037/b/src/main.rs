// -*- coding:utf-8-unix -*-

// 12.5min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }

    let mut grp = (0..n).collect::<Vec<usize>>();
    let mut passes = vec![true; n];
    let mut is_found = vec![false; n];
    // BFS で親以外の既出頂点を踏まない
    for i in 0..n {
        if is_found[i] {
            continue;
        }

        let cur_grp = grp[i];
        let mut vdq = VecDeque::new();
        vdq.push_back((i, None));
        is_found[i] = true;
        while let Some(cur) = vdq.pop_front() {
            for e in &edges[cur.0] {
                if is_found[*e] {
                    if cur.1 != Some(*e) {
                        passes[cur_grp] = false;
                    }
                    continue;
                }

                vdq.push_back((*e, Some(cur.0)));
                is_found[*e] = true;
                passes[grp[*e]] = false;
                grp[*e] = cur_grp;
            }
        }
    }
    // println!("{:?}", grp);
    // println!("{:?}", passes);
    // println!("{:?}", is_found);

    println!("{}", passes.iter().filter(|&a| *a).count());
}
