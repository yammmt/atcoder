use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

static DUMMY: isize = std::isize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        axy: (usize, usize),
        bxy: (usize, usize),
        sn: [Chars; n],
    }

    // 方向をもたせて 0-1BFS をするとなぜか異様に遅くなる

    // 個々のマスから移動できる場所を全探索すると O(n^3)
    // 移動先を高速に探索しなければならない
    // 四方向への移動にここまで探索したフラグを入れる？

    let mut to_plus_plus = vec![vec![false; n]; n];
    let mut to_plus_minus = vec![vec![false; n]; n];
    let mut to_minus_plus = vec![vec![false; n]; n];
    let mut to_minus_minus = vec![vec![false; n]; n];

    let mut q = VecDeque::new();
    // (cost, x, y)
    q.push_back((0, axy.0 - 1, axy.1 - 1));
    let mut costs = vec![vec![DUMMY; n]; n];
    while let Some(cur) = q.pop_front() {
        let x = cur.1;
        let y = cur.2;
        if costs[x][y] != DUMMY {
            continue;
        }

        // TODO: 四方向判定を集約したい
        // うまそうな他者: https://atcoder.jp/contests/abc246/submissions/45813872
        costs[x][y] = cur.0;
        for d in 1..=(n - 1 - x).min(n - 1 - y) {
            let xd = x + d;
            let yd = y + d;
            if to_plus_plus[xd][yd] || sn[xd][yd] == '#' {
                break;
            }

            to_plus_plus[xd][yd] = true;
            q.push_back((cur.0 + 1, xd, yd));
        }

        for d in 1..=(n-1-x).min(y) {
            let xd = x + d;
            let yd = y - d;
            if to_plus_minus[xd][yd] || sn[xd][yd] == '#' {
                break;
            }

            to_plus_minus[xd][yd] = true;
            q.push_back((cur.0 + 1, xd, yd));
        }

        for d in 1..=x.min(n-1-y) {
            let xd = x - d;
            let yd = y + d;
            if to_minus_plus[xd][yd] || sn[xd][yd] == '#' {
                break;
            }

            to_minus_plus[xd][yd] = true;
            q.push_back((cur.0 + 1, xd, yd));
        }

        for d in 1..=x.min(y) {
            let xd = x - d;
            let yd = y - d;
            if to_minus_minus[xd][yd] || sn[xd][yd] == '#' {
                break;
            }

            to_minus_minus[xd][yd] = true;
            q.push_back((cur.0 + 1, xd, yd));
        }
    }

    println!(
        "{}",
        if costs[bxy.0 - 1][bxy.1 - 1] == DUMMY {
            -1
        } else {
            costs[bxy.0 - 1][bxy.1 - 1]
        }
    );
}
