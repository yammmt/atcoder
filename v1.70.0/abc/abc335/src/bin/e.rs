use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        uvm: [(Usize1, Usize1); m],
    }

    let mut edges = vec![vec![]; n];
    for (u, v) in uvm {
        edges[u].push(v);
        edges[v].push(u);
    }

    // 種類数と最後に通った頂点の数字だけを覚えておけばよい
    // 木構造であれば素直にできそうだが今回はそうではない
    // 頂点をまわる順番を数字小さい順で見てやればよさそう
    // 探索の始点は 1 固定

    // やたらペナ多いけどなんで？ => 公式解説 DP だった

    let mut v_score = vec![0; n];
    // 頂点の数字, 今の種類数, 頂点) では
    // これで頂点数字小 -> 得点大順にソートされる
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(an[0]), 1, 0));
    while let Some((Reverse(a), score, i)) = heap.pop() {
        // println!("i: {i}");
        if v_score[i] != 0 {
            continue;
        }

        v_score[i] = score;
        for &v in &edges[i] {
            match a.cmp(&an[v]) {
                Ordering::Less => heap.push((Reverse(an[v]), score + 1, v)),
                Ordering::Equal => heap.push((Reverse(an[v]), score, v)),
                Ordering::Greater => {}
            }
        }
    }

    println!("{}", v_score[n - 1]);
}
