use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        cn: [usize; n],
        abn1: [(usize, usize); n - 1],
    }
    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut ans = vec![];
    let mut color_num = vec![0; 100_001];
    let mut visited = vec![false; n];
    let mut vdq = VecDeque::new();
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_back() {
        if visited[cur] {
            color_num[cn[cur]] -= 1;
            continue;
        }

        if color_num[cn[cur]] == 0 {
            ans.push(cur + 1);
        }
        color_num[cn[cur]] += 1;
        visited[cur] = true;
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
