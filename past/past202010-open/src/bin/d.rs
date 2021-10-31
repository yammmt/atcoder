// 苦手

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut sn: Chars,
    }

    // マス 1 を埋めるように左分身する
    let mut ans_x = 0;
    while sn.first().unwrap() != &'#' {
        ans_x += 1;
        for i in 0..n - 1 {
            if sn[i + 1] == '#' {
                sn[i] = '#';
            }
        }
    }
    // println!("{:?}", sn);

    // マス N を埋めるように右分身する
    let mut ans_y = 0;
    while sn.last().unwrap() != &'#' {
        ans_y += 1;
        for i in (1..n).rev() {
            if sn[i - 1] == '#' {
                sn[i] = '#';
            }
        }
    }
    // println!("{:?}", sn);
    // println!("{} {}", ans_x, ans_y);

    // 埋まっていないマス分追加で分身する
    // 連続する '.' の最大値分どちらかに足す
    let mut ans_xy = 0;
    let mut cur = 0;
    for s in &sn {
        if *s == '.' {
            cur += 1;
        } else {
            cur = 0;
        }
        ans_xy = ans_xy.max(cur);
    }
    ans_x += ans_xy;

    println!("{} {}", ans_x, ans_y);
}
