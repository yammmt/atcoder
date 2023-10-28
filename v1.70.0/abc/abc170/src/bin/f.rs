// ジャッジ合ってる？嘘解じゃない？

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

static DUMMY: usize = std::usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let hi = h as isize;
    let wi = w as isize;

    let mut score = vec![vec![DUMMY; w]; h];
    let mut que = VecDeque::new();
    // (cost, (x, y))
    que.push_back((0, (x1 - 1, y1 - 1)));
    score[x1 - 1][y1 - 1] = 0;
    while let Some((p, (x, y))) = que.pop_front() {
        if p != score[x][y] {
            // 別経路で折り返しても自身以下のスコアが取れる
            continue;
        }

        // println!("{x}, {y}");
        for d in &dir {
            let mut i = 1;
            let mut xn = x as isize + d.0;
            let mut yn = y as isize + d.1;
            while i <= k
                && xn >= 0
                && yn >= 0
                && xn < hi
                && yn < wi
                // だめでは？
                // 移動制限があるのでもう 1 マス動くとスコアが更新できる可能性がある
                && p < score[xn as usize][yn as usize]
                && shw[xn as usize][yn as usize] == '.'
            {
                // XXX: なんで通る？
                // 計算量は始点が HW 通り, 各点について四方向 K マス舐めるのは
                // 変わらないから結局 O(HWK) にならない？
                // 跡から来た探索が開始早々修了するから？
                // println!("  + {xn} {yn}");
                if p + 1 < score[xn as usize][yn as usize] {
                    score[xn as usize][yn as usize] = p + 1;
                    que.push_back((p + 1, (xn as usize, yn as usize)));
                }
                xn += d.0;
                yn += d.1;
                i += 1;
            }
        }
    }
    // for s in &score {
    //     println!("{:?}", s);
    // }

    if score[x2 - 1][y2 - 1] == DUMMY {
        println!("-1");
    } else {
        println!("{}", score[x2 - 1][y2 - 1]);
    }
}
