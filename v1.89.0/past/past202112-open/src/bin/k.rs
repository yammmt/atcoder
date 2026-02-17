use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

const UNVISITED: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        k: usize,
        ak: [Usize1; k],
        uvm: [(Usize1, Usize1); m],
        stq: [(Usize1, Usize1); q],
    }

    let mut edges = vec![vec![]; n];
    for (u, v) in uvm {
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut costs = vec![vec![UNVISITED; n]; k];
    for (i, a) in ak.iter().enumerate() {
        let mut que = VecDeque::new();
        que.push_back((*a, 0));
        while let Some((vcur, cost_cur)) = que.pop_front() {
            if costs[i][vcur] != UNVISITED {
                continue;
            }

            costs[i][vcur] = cost_cur;

            for &vnext in &edges[vcur] {
                if costs[i][vnext] == UNVISITED {
                    que.push_back((vnext, cost_cur + 1));
                }
            }
        }
    }

    for (s, t) in stq {
        let mut ans = usize::MAX;
        for i in 0..k {
            ans = ans.min(costs[i][s] + costs[i][t]);
        }

        println!("{ans}");
    }
}
