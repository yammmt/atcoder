// 解説の長さと難易度に開きがあるように見える

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m],
    }
    let d = 998_244_353;
    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }

    let mut ans = 1;
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut vdq = VecDeque::new();
        let mut used_vertexes = 0;
        let mut used_edges = 0;
        vdq.push_back(i);
        visited[i] = true;
        while let Some(cur) = vdq.pop_front() {
            used_vertexes += 1;
            for v in &edges[cur] {
                used_edges += 1;
                if !visited[*v] {
                    vdq.push_back(*v);
                    visited[*v] = true;
                }
            }
        }

        if 2 * used_vertexes == used_edges {
            ans = ans * 2 % d;
        } else {
            println!("0");
            return;
        }
    }

    println!("{}", ans);
}
