// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, u64); q],
    }

    let mut edges = vec![vec![]; n + 1];
    for i in &ab {
        edges[i.0].push(i.1);
        edges[i.1].push(i.0);
    }

    let mut pts = vec![0; n + 1];
    for i in &px {
        pts[i.0] += i.1;
    }
    // println!("{:?}", pts);

    let mut ans = vec![0; n + 1];
    let mut vdq = VecDeque::new();
    vdq.push_back((1, 0));
    let mut visited = vec![false; n + 1];
    while !vdq.is_empty() {
        let cn = vdq.pop_front().unwrap();
        visited[cn.0] = true;
        ans[cn.0] = cn.1 + pts[cn.0];

        for e in &edges[cn.0 as usize] {
            if visited[*e] {
                continue;
            }

            vdq.push_back((*e, ans[cn.0]));
        }
    }

    for i in 1..n + 1 {
        print!("{}", ans[i]);
        if i == n {
            println!("");
        } else {
            print!(" ");
        }
    }
}
