use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, usize); m],
    }

    // どの二点間も行き来できる, であればクラスカル方で最小全域木
    // 今回は都市 1 を起点としているから違う
    // 単純に 1 を起点にダイクストラするだけ？
    // ダイクストラ中に辺の情報を答えに代入すれば勝手に最短経路木になるはずだけどなんで WA?
    // => ダイクストラが書けていなかったから (現在までのコストを捨てていた)

    let mut edges = vec![vec![]; n];
    for (i, abc) in abcm.iter().enumerate() {
        let a = abc.0 - 1;
        let b = abc.1 - 1;
        let c = abc.2;
        edges[a].push((c, b, i + 1));
        edges[b].push((c, a, i + 1));
    }

    let mut ans = Vec::with_capacity(n - 1);
    let mut bh = BinaryHeap::new();
    let mut visited = vec![false; n];
    visited[0] = true;
    for &e in &edges[0] {
        bh.push(Reverse(e));
    }
    while let Some(Reverse(cur)) = bh.pop() {
        if visited[cur.1] {
            continue;
        }

        visited[cur.1] = true;
        ans.push(cur.2);
        for e in &edges[cur.1] {
            if visited[e.1] {
                continue;
            }

            bh.push(Reverse((e.0 + cur.0, e.1, e.2)));
        }
    }

    // assert!(ans.len() == n - 1);
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
