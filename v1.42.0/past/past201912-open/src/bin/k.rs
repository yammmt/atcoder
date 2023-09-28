// https://www.youtube.com/watch?v=1V45kF40zHc

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        pn: [isize; n],
        q: usize,
        abq: [(usize, usize); q],
    }

    let mut boss = 0;
    let mut children = vec![vec![]; n + 1];
    for (i, p) in pn.iter().enumerate() {
        if *p == -1 {
            boss = i + 1;
            continue;
        }

        children[*p as usize].push(i + 1);
    }
    // println!("children: {:?}", children);

    let mut pos = vec![std::usize::MAX; n + 1];
    let mut from_idx = vec![std::usize::MAX; n + 1];
    let mut to_idx = vec![std::usize::MAX; n + 1];
    let mut vdq = VecDeque::new();
    vdq.push_back(boss);
    let mut idx = 0;
    while let Some(cur) = vdq.pop_back() {
        // println!("cur: {}", cur);
        // 行きがけなら true
        match from_idx[cur] == std::usize::MAX {
            true => from_idx[cur] = idx,
            false => {
                to_idx[cur] = idx;
                continue;
            }
        }

        pos[cur] = idx;
        vdq.push_back(cur);
        for c in &children[cur] {
            vdq.push_back(*c as usize);
        }
        // println!("{:?}", vdq);

        idx += 1;
    }
    // println!("pos: {:?}", pos);
    // println!("from: {:?}", from_idx);
    // println!("to: {:?}", to_idx);

    for ab in &abq {
        // ab.0 != ab.1
        println!(
            "{}",
            match from_idx[ab.1] < pos[ab.0] && to_idx[ab.1] > pos[ab.0] {
                true => "Yes",
                false => "No",
            }
        );
    }
}
