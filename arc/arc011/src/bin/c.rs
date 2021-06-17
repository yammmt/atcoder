// 28.5min
// 解法はすぐだが実装, 特にデバッグに手間取った

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        c_first: Chars,
        c_last: Chars,
        n: usize,
        mut sn: [Chars; n],
    }
    if c_first == c_last {
        println!("0");
        println!("{}", c_first.iter().collect::<String>());
        println!("{}", c_first.iter().collect::<String>());
        return;
    }

    sn.push(c_first.clone());
    sn.push(c_last.clone());
    sn.sort_unstable();
    sn.dedup();
    let sn = sn;
    // println!("{:?}", sn);

    let mut first_idx = 0;
    let mut last_idx = 0;
    for (i, s) in sn.iter().enumerate() {
        if *s == c_first {
            first_idx = i;
        } else if *s == c_last {
            last_idx = i;
        }
    }

    let mut edges = vec![vec![]; sn.len()];
    for i in 0..sn.len() {
        for j in i + 1..sn.len() {
            let mut diff_num = 0;
            for k in 0..sn[0].len() {
                if sn[i][k] != sn[j][k] {
                    diff_num += 1;
                }
            }
            if diff_num == 1 {
                edges[i].push(j);
                edges[j].push(i);
            }
        }
    }
    // println!("{:?}", edges);

    let mut comes_from = vec![None; sn.len()];
    let mut q = VecDeque::new();
    q.push_back(first_idx);
    while let Some(cur) = q.pop_front() {
        for e in &edges[cur] {
            if comes_from[*e].is_some() || *e == first_idx {
                continue;
            }

            comes_from[*e] = Some(cur);
            q.push_back(*e);
        }
    }
    // println!("{:?}", comes_from);

    if comes_from[last_idx].is_none() {
        println!("-1");
        return;
    }

    let mut ans = vec![c_last.iter().collect::<String>()];
    let mut i = last_idx;
    while let Some(cur) = comes_from[i] {
        ans.push(sn[cur].iter().collect::<String>());
        i = cur;
    }
    ans.reverse();
    println!("{}", ans.len() - 2);
    for a in &ans {
        println!("{}", a);
    }
}
