// :fu: 方針が冗長でバグを埋め込んだ
// https://drken1215.hatenablog.com/entry/2020/11/27/012048
// サンプルほしい 一回全部 "No" で提出すれば存在しない場合があるかは WA と引き換えに絞れる

use proconio::input;
use std::collections::VecDeque;

const PENDING: usize = 999_999_999;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvcm: [(usize, usize, usize); m],
    }
    let mut edges = vec![vec![]; n + 1];
    for uvc in &uvcm {
        edges[uvc.0].push((uvc.1, uvc.2));
        edges[uvc.1].push((uvc.0, uvc.2));
    }
    // println!("{:?}", edges);

    let mut ans = vec![PENDING; n + 1];
    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    ans[1] = 1;
    while let Some(cur) = vdq.pop_front() {
        for e in &edges[cur] {
            if ans[e.0] != PENDING {
                continue;
            }

            if e.1 == ans[cur] {
                // 0 が入ってしまうと WA
                ans[e.0] = ((ans[cur] + 1) % n).max(1);
            } else {
                ans[e.0] = e.1;
            }
            vdq.push_back(e.0);
        }
    }


    for a in ans.iter().skip(1) {
        println!("{}", a);
    }
}
