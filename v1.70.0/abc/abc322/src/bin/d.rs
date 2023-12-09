// マスを座標集合でもつ方針
// https://atcoder.jp/contests/abc322/editorial/7369
// Rust で型変換が入るととても辛い

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn is_yes(pos: &[Vec<(isize, isize)>], turn: [usize; 3], pmove: &[(isize, isize); 3]) -> bool {
    let mut filled = HashSet::new();
    for (i, vp) in pos.iter().enumerate() {
        // 回転
        // 最小値を記憶すると左上の座標を (0, 0) として使える
        let mut x_min = isize::MAX / 2;
        let mut y_min = isize::MAX / 2;
        let mut pos_moved = vec![];
        for p in vp {
            let mut x = p.0;
            let mut y = p.1;
            for _ in 0..turn[i] {
                let tmp = x;
                x = y;
                y = -tmp;
            }
            pos_moved.push((x, y));
            x_min = x_min.min(x);
            y_min = y_min.min(y);
        }
        // 平行移動
        for (x, y) in pos_moved {
            filled.insert((x + pmove[i].0 - x_min, y + pmove[i].1 - y_min));
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            if !filled.contains(&(i, j)) {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        p0: [Chars; 4],
        p1: [Chars; 4],
        p2: [Chars; 4],
    }

    // 平行移動の候補は最大で 4x4=16 per Polyomino
    // 回転は 4 通り
    // 合わせて 1 Polyomino に対し 16x4 = 64 通りの置き方がある
    // これが 3 つあると 64x3 = 192 通りにしかならず, 全探索ができそう
    // 実装はつらめ

    let mut sharp_num = 0;
    let mut pos = vec![vec![]; 3];
    for (i, p) in [p0, p1, p2].iter().enumerate() {
        for j in 0..4 {
            for k in 0..4 {
                if p[j][k] == '#' {
                    pos[i].push((j as isize, k as isize));
                    sharp_num += 1;
                }
            }
        }
    }
    // 最初に弾かなければ後の判定が辛くなる, 重ねて置かれると通ってしまう (例 5)
    if sharp_num != 16 {
        println!("No");
        return;
    }

    // 回転
    let mut turn_num = vec![];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                turn_num.push([i, j, k]);
            }
        }
    }
    // 平行移動
    let mut parallel_move_each = vec![];
    for i in 0..4 {
        for j in 0..4 {
            parallel_move_each.push((i as isize, j as isize));
        }
    }
    let pm_len = parallel_move_each.len();
    let mut parallel_move = vec![];
    for i in 0..pm_len {
        for j in 0..pm_len {
            for k in 0..pm_len {
                parallel_move.push([
                    parallel_move_each[i],
                    parallel_move_each[j],
                    parallel_move_each[k],
                ]);
            }
        }
    }

    for t in turn_num {
        for pm in &parallel_move {
            if is_yes(&pos, t, pm) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
