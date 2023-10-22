use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m],
    }

    // 強連結成分でなくても解けない？

    let mut edges_f = vec![vec![]; n];
    let mut edges_b = vec![vec![]; n];
    for uv in uvm {
        let u = uv.0 - 1;
        let v = uv.1 - 1;
        edges_f[u].push(v);
        edges_b[v].push(u);
    }

    let mut order = vec![];
    let mut order_saved = vec![false; n];
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut vdq = VecDeque::new();
        vdq.push_back(start);
        while let Some(cur) = vdq.pop_back() {
            if visited[cur] {
                if !order_saved[cur] {
                    order_saved[cur] = true;
                    order.push(cur);
                }
                continue;
            }

            visited[cur] = true;
            vdq.push_back(cur);
            for e in &edges_f[cur] {
                if !visited[*e] {
                    vdq.push_back(*e);
                }
            }
        }
    }

    order.reverse();
    visited = vec![false; n];
    for start in &order {
        if visited[*start] {
            continue;
        }

        let mut vdq = VecDeque::new();
        let mut hs = HashSet::new();
        vdq.push_back(*start);
        while let Some(cur) = vdq.pop_back() {
            hs.insert(cur);
            if visited[cur] {
                continue;
            }

            visited[cur] = true;
            for e in &edges_b[cur] {
                if !visited[*e] {
                    vdq.push_back(*e);
                }
            }
        }
        if hs.len() > 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
