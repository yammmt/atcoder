// 41min

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        h: usize,
        w: usize,
        _k: usize,
        s: [Chars; h],
    }

    let mut ans = vec![vec![0; w]; h];
    let mut cur = 1;
    let mut empty_line = HashSet::new();
    for i in 0..h {
        let mut is_strawberry_appeared = false;
        let mut is_empty_line = true;
        for j in 0..w {
            if s[i][j] == '#' {
                if is_strawberry_appeared {
                    cur += 1;
                }
                is_strawberry_appeared = true;
                is_empty_line = false;
            }
            ans[i][j] = cur;
        }
        if is_empty_line {
            empty_line.insert(i);
        } else {
            cur += 1;
        }
    }

    // 上から下に空行 (strawberry のない) 箇所を補完する
    // 1 行目が埋まっていることを前提とさえできれば以後は一周舐めるだけ
    // println!("{:?}", empty_line);
    if empty_line.contains(&0) {
        // 1 行目の補完
        for i in 1..h {
            if !empty_line.contains(&i) {
                for j in 0..w {
                    ans[0][j] = ans[i][j];
                }
                break;
            }
        }
    }
    for i in 1..h {
        if empty_line.contains(&i) {
            for j in 0..w {
                ans[i][j] = ans[i - 1][j];
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
