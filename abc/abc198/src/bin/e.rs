// :fu: 21-04 これは解きたい

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        ci: [usize; n],
        abn1: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    // println!("{:?}", edges);

    let mut ans = vec![];
    let mut vdq = VecDeque::new();
    let mut color_cnt = vec![0; 100_001];
    let mut visited = vec![false; n];
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_back() {
        if visited[cur] {
            color_cnt[ci[cur]] -= 1;
            continue;
        }

        visited[cur] = true;
        color_cnt[ci[cur]] += 1;
        if color_cnt[ci[cur]] == 1 {
            ans.push(cur + 1);
        }
        vdq.push_back(cur);
        for e in &edges[cur] {
            if !visited[*e] {
                vdq.push_back(*e);
            }
        }
    }

    ans.sort_unstable();
    for a in &ans {
        println!("{}", a);
    }
}
