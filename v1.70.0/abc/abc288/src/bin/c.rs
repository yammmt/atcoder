use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(Usize1, Usize1); m],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in abm {
        edges[a].push(b);
        edges[b].push(a);
    }
    let edges = edges;

    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut que = VecDeque::new();
        let mut edge_num = 0;
        let mut visited_num = 0;
        que.push_back(i);
        while let Some(v) = que.pop_front() {
            edge_num += 1;
            if visited[v] {
                continue;
            }

            visited[v] = true;
            for vv in &edges[v] {
                que.push_back(*vv);
            }
            visited_num += 1;
        }
        edge_num /= 2;
        ans += edge_num - (visited_num - 1);
    }

    println!("{ans}");
}
