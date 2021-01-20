// 辺を指定されるため、操作対象の点の距離は必ず 1 になる
// t による分岐は swap かけて済ませてやれば実装が数行分軽くなる

// 二点間の距離が不定の場合は部分木判定やらオイラーツアーやらな単語が出てくる
// 考察も実装も重くなる
// https://atcoder.jp/contests/abc187/submissions/19164468

use proconio::input;
use std::collections::VecDeque;

#[allow(clippy::many_single_char_names)]
fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
        q: usize,
        texq: [(usize, usize, i64); q],
    }

    let mut edges = vec![vec![]; n + 1];
    for ab in &abn {
        edges[ab.0].push(ab.1);
        edges[ab.1].push(ab.0);
    }

    let mut ranks = vec![std::usize::MAX; n + 1];
    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    let mut rank = 0;
    while let Some(cur) = vdq.pop_back() {
        if ranks[cur] != std::usize::MAX {
            continue;
        }

        ranks[cur] = rank;
        rank += 1;
        for e in &edges[cur] {
            if ranks[*e] == std::usize::MAX {
                vdq.push_back(*e);
            }
        }
    }

    let mut c = vec![0; n + 1];
    for tex in &texq {
        let mut a = abn[tex.1 - 1].0;
        let mut b = abn[tex.1 - 1].1;
        if tex.0 == 2 {
            std::mem::swap(&mut a, &mut b);
        }

        if ranks[a] < ranks[b] {
            c[1] += tex.2;
            c[b] -= tex.2;
        } else {
            c[a] += tex.2;
        }
    }

    let mut anss = vec![std::usize::MAX; n + 1];
    let mut vdq = VecDeque::new();
    vdq.push_back((1, 0i64));
    while let Some(cur) = vdq.pop_back() {
        if anss[cur.0] != std::usize::MAX {
            continue;
        }

        let pts = cur.1 + c[cur.0];
        anss[cur.0] = pts as usize;
        for e in &edges[cur.0] {
            if anss[*e] == std::usize::MAX {
                vdq.push_back((*e, pts));
            }
        }
    }

    for ans in anss.iter().skip(1) {
        println!("{}", ans);
    }
}
