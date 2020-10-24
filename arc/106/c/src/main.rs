// -*- coding:utf-8-unix -*-

// LR は正 (0 含まない), 座標の重なり禁止
// 制約は読めとこれほど...

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
    }

    if n == 1 {
        if m == 0 {
            println!("1 2");
        } else {
            println!("-1");
        }
        return;
    }

    // L 昇順 (青木) に勝ち目はない
    if m < 0 || m > (n as i32 - 2) {
        println!("-1");
        return;
    }

    let mut written = 0;
    if m > 0 {
        println!("1 {}", 2 * (m + 2) + 1);
        written += 1;
        let mut ks = 2;
        for _ in 0..m + 1 {
            println!("{} {}", ks ,ks + 1);
            ks += 2;
            written += 1;
        }
    }
    // println!("");
    // 配置点は問わないので分岐も要らない
    let mut ks = 2 * (m + 1) + 4;
    for _ in 0..n - written {
        println!("{} {}", ks, ks + 1);
        ks += 2;
    }
}
