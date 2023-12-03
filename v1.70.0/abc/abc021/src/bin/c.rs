use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

const DUMMY: usize = usize::MAX / 4;
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        m: usize,
        xym: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for (x, y) in xym {
        edges[x - 1].push(y - 1);
        edges[y - 1].push(x - 1);
    }

    let mut costs = vec![DUMMY; n];
    costs[a - 1] = 0;
    let mut num_of_paths = vec![0; n];
    num_of_paths[a - 1] = 1;
    let mut visited = vec![false; n];
    // (頂点, cost)
    let mut que = VecDeque::from([(a - 1, 0)]);
    while let Some((v_cur, cost_cur)) = que.pop_front() {
        if visited[v_cur] {
            continue;
        }

        visited[v_cur] = true;
        let cost_nxt = cost_cur + 1;
        for &v in &edges[v_cur] {
            if visited[v] || cost_nxt > costs[v] {
                continue;
            } else if cost_nxt == costs[v] {
                num_of_paths[v] = (num_of_paths[v] + num_of_paths[v_cur]) % MOD;
            } else {
                // cost が DUMMY の場合のみ
                costs[v] = cost_nxt;
                num_of_paths[v] = num_of_paths[v_cur];
            }

            que.push_back((v, cost_nxt));
        }
    }
    // println!("{:?}", num_of_paths);

    println!("{}", num_of_paths[b - 1]);
}
