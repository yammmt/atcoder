// 数問 捨て 実装がまったくわからん
// https://note.com/syamashi/n/ndd5f58b2f9ea

use proconio::input;
use std::mem;

fn main() {
    input! {
        mut x: i64,
        mut y: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let candidates = [[a, b, c], [b, c, a], [c, a, b]];

    for cur in &candidates {
        for _ in 0..2 {
            // X 軸に三つの長方形が触れている
            let required_x = cur.iter().map(|p| (p + y - 1) / y).sum::<i64>();
            if required_x <= x {
                println!("Yes");
                return;
            }
            // X 軸に一つの長方形が触れている
            let used_y = (cur[0] + x - 1) / x;
            if y - used_y > 0 {
                let required_x = cur
                    .iter()
                    .skip(1)
                    .map(|p| (p + y - used_y - 1) / (y - used_y))
                    .sum::<i64>();
                if required_x <= x {
                    println!("Yes");
                    return;
                }
            }
            mem::swap(&mut x, &mut y);
        }
    }

    println!("No");
}
