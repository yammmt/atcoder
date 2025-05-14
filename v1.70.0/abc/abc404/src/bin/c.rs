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

    // 必要条件ではあるが, これを省いても問題なし
    if m != n {
        println!("No");
        return;
    }

    // 適当な始点から開始して, 一周して帰ってこれればよし
    let mut que = VecDeque::new();
    let mut visited = vec![false; n];
    // (現在地, 移動元)
    que.push_back((0, 0));
    while let Some((v_cur, v_prev)) = que.pop_front() {
        if v_cur == 0 && visited[0] {
            if visited.iter().filter(|&t| *t).count() == n {
                println!("Yes");
            } else {
                println!("No");
            }
            return;
        }

        visited[v_cur] = true;
        if edges[v_cur].len() != 2 {
            println!("No");
            return;
        }

        for &v_next in &edges[v_cur] {
            if v_next != v_prev {
                que.push_back((v_next, v_cur));
                break;
            }
        }
    }

    // 0 から動けないとここで終わる
    println!("No");
}
