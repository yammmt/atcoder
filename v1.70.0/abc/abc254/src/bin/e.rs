use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(Usize1, Usize1); m],
        q: usize,
        xkq: [(Usize1, usize); q],
    }

    // 閉路となり得る (例 1)

    let mut edges = vec![vec![]; n];
    for (a, b) in abm {
        edges[a].push(b);
        edges[b].push(a);
    }

    for (x, k) in xkq {
        let mut visited = HashMap::new();
        let mut que = VecDeque::new();
        // (点, 距離)
        que.push_back((x, 0));
        while let Some((cur_x, cur_k)) = que.pop_front() {
            if visited.contains_key(&cur_x) {
                continue;
            }

            visited.insert(cur_x, cur_k);
            if cur_k == k {
                continue;
            }

            for v in &edges[cur_x] {
                que.push_back((*v, cur_k + 1));
            }
        }

        let mut ans = 0;
        for i in visited.keys() {
            ans += *i + 1;
        }
        println!("{ans}");
    }
}
