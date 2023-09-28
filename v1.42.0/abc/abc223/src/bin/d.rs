// 20.5min トポロジカルソート

use proconio::input;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    // 辞書順最小を出したいので選択可能な最小の数を貪欲に取っていけば良い
    // すべての数に対し先に出てこなければならない数の集合を用意してやって Priority Queue に突っ込む
    // 途中で選べる数がなくなった場合は -1 出力
    // 言い換えれば有向グラフを作ってループができてしまったら -1 出力

    // 何をブロックしているか？ と 何にブロックされているか？ の両方の情報を管理せねばならぬ
    // さもなくば TLE
    let mut blocks = vec![HashSet::new(); n + 1];
    let mut blocked_by = vec![HashSet::new(); n + 1];
    for ab in &abm {
        // A_i は B_i を阻止する
        blocks[ab.0].insert(ab.1);
        // B_i を阻止するのが A_i
        blocked_by[ab.1].insert(ab.0);
    }
    // バグらせそうなので束縛し直す
    let blocks = blocks;

    let mut available_nums = BinaryHeap::new();
    for (i, hs) in blocked_by.iter().skip(1).enumerate() {
        if hs.is_empty() {
            available_nums.push(-(i as isize + 1));
        }
    }

    let mut ans = vec![];
    while let Some(cur) = available_nums.pop() {
        let ucur = -cur as usize;
        ans.push(ucur);
        for released in &blocks[ucur] {
            blocked_by[*released].remove(&ucur);
            if blocked_by[*released].is_empty() {
                available_nums.push(-(*released as isize));
            }
        }
    }

    if ans.len() != n {
        println!("-1");
    } else {
        for (i, a) in ans.iter().enumerate() {
            print!("{}", a);
            if i == n - 1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
}
