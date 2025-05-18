use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

const DUMMY: isize = isize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(Usize1, Usize1, isize); m],
    }
    let mut edges = vec![vec![]; n];
    for &(a, b, c) in &abcm {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    // x >= 0
    // 条件が x-p なら x は p 以上
    // 条件が q-x なら x は q 以下
    // 最終的には p<=x<=q

    let mut posi = vec![0; n];
    let mut nega = vec![0; n];
    let mut exists_posi = vec![false; n];
    let mut exists_nega = vec![false; n];
    let mut lower_bound = 0;
    let mut upper_bound = DUMMY;

    let mut que = VecDeque::new();
    exists_posi[0] = true;
    que.push_back(0);
    while let Some(v_cur) = que.pop_front() {
        for &(v_next, cost) in &edges[v_cur] {
            let mut updated = false;
            // v_cur が pos[v_cur] + x
            // x <= c - positive[v_cur]
            if exists_posi[v_cur] && !exists_nega[v_next] {
                exists_nega[v_next] = true;
                nega[v_next] = cost - posi[v_cur];
                upper_bound = upper_bound.min(cost - posi[v_cur]);
                updated = true;
            }
            // v_cur が nega[v_cur] - x
            // x >= nega[v_cur] - c
            if exists_nega[v_cur] && !exists_posi[v_next] {
                exists_posi[v_next] = true;
                posi[v_next] = cost - nega[v_cur];
                lower_bound = lower_bound.max(nega[v_cur] - cost);
                updated = true;
            }

            if updated {
                que.push_back(v_next);
            }
        }
    }

    if lower_bound > upper_bound {
        println!("-1");
        return;
    }

    let mut ans = vec![0; n];
    let mut x = 0;

    // nega[i] - x = posi[i] + x より, 両方の情報があれば x の値は一意に定まる
    for i in 0..n {
        if exists_posi[i] && exists_nega[i] {
            if (nega[i] - posi[i]) % 2 != 0 {
                println!("-1");
                return;
            }

            x = (nega[i] - posi[i]) / 2;
            break;
        }
    }
    // x==0 はありうるのでは, 全辺 c=0 入力が存在するので
    // => 全辺 c=0 なら upper_bound=0 だから変わらず
    if x == 0 {
        x = upper_bound;
    }

    for i in 0..n {
        if exists_posi[i] {
            ans[i] = posi[i] + x;
        } else if exists_nega[i] {
            ans[i] = nega[i] - x;
        }
        if ans[i] < 0 {
            println!("-1");
            return;
        }
    }

    // 再確認が必要: 両方向の情報をもつ x が一点であるとは限らないから, だけ？
    for (a, b, c) in abcm {
        if ans[a] + ans[b] != c {
            println!("-1");
            return;
        }
    }

    for a in ans {
        println!("{a}");
    }
}
