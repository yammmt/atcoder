use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut connected = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            x: usize,
            u: Usize1,
            v: Usize1,
        }
        if x == 1 {
            if connected[u].contains(&v) {
                connected[u].remove(&v);
                connected[v].remove(&u);
            } else {
                connected[u].insert(v);
                connected[v].insert(u);
            }
        } else {
            let mut visited = vec![false; n];
            let mut que = VecDeque::new();
            que.push_back(u);
            while let Some(vcur) = que.pop_front() {
                if visited[vcur] {
                    continue;
                }

                visited[vcur] = true;
                if vcur == v {
                    break;
                }

                for &vnext in &connected[vcur] {
                    // 一々判定した方がちょっと速くなりそう
                    que.push_back(vnext);
                }
            }

            println!("{}", if visited[v] { "Yes" } else { "No" });
        }
    }
}
