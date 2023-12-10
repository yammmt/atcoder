// マスを座標集合でもつ方針
// https://atcoder.jp/contests/abc322/editorial/7369
// Rust で型変換が入るととても辛い

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn is_yes(vp: &[Vec<(isize, isize)>], t: &[usize; 3], pm: &[(isize, isize); 3]) -> bool {
    let mut filled = HashSet::new();
    for (i, p) in vp.iter().enumerate() {
        let mut p_turned = vec![];
        let mut x_min = 999;
        let mut y_min = 999;
        for pp in p {
            let mut x = pp.0;
            let mut y = pp.1;
            for _ in 0..t[i] {
                let tmp = x;
                x = y;
                y = -tmp;
            }
            x_min = x_min.min(x);
            y_min = y_min.min(y);
            p_turned.push((x, y));
        }

        for pp in p_turned {
            let inserted = (pp.0 - x_min + pm[i].0, pp.1 - y_min + pm[i].1);
            filled.insert(inserted);
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

    let vp = [p0, p1, p2];

    let mut sharp_cnt = 0;
    for p in &vp {
        for i in 0..4 {
            for j in 0..4 {
                if p[i][j] == '#' {
                    sharp_cnt += 1;
                }
            }
        }
    }
    if sharp_cnt != 16 {
        println!("No");
        return;
    }

    let mut vp_pts = vec![];
    for p in vp {
        let mut cur = vec![];
        for i in 0..4 {
            for j in 0..4 {
                if p[i][j] == '#' {
                    cur.push((i as isize, j as isize));
                }
            }
        }
        vp_pts.push(cur);
    }

    let mut turn_num = vec![];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                turn_num.push([i, j, k]);
            }
        }
    }

    let mut para_move = vec![];
    for i in 0..4 {
        for j in 0..4 {
            para_move.push((i as isize, j as isize));
        }
    }
    let para_move_len = para_move.len();

    let mut para_moves = vec![];
    for i in 0..para_move_len {
        for j in 0..para_move_len {
            for k in 0..para_move_len {
                para_moves.push([para_move[i], para_move[j], para_move[k]]);
            }
        }
    }

    for t in &turn_num {
        for p in &para_moves {
            if is_yes(&vp_pts, t, p) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
