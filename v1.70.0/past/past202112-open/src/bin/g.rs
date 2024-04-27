use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // vector の削除するのとどっちがマシな計算量になるかしら
    // 実装はこっちが楽だと思うけど
    // => editorial は隣接行列方針
    let mut connected = vec![vec![false; n]; n];
    for _ in 0..q {
        input! {
            a: usize,
            u: Usize1,
            v: Usize1,
        }

        if a == 1 {
            connected[u][v] = !connected[u][v];
            connected[v][u] = !connected[v][u];
        } else {
            let mut que = VecDeque::new();
            let mut visited = vec![false; n];
            que.push_back(u);
            while let Some(cur) = que.pop_back() {
                if visited[cur] {
                    continue;
                }

                visited[cur] = true;
                for i in 0..n {
                    if connected[cur][i] {
                        que.push_back(i);
                    }
                }
            }

            println!("{}", if visited[v] { "Yes" } else { "No" });
        }
    }
}
