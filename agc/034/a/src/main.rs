// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut is_fail = true;
    if c > d {
        // A/B がすれ違う場所があるか？
        // a < b より b 基準で前後 1 マス
        for i in b - 1..d {
            if vc[i as usize] == '.' && vc[i as usize - 1] == '.' && vc[i as usize + 1] == '.' {
                is_fail = false;
                break;
            }
        }
        if is_fail {
            println!("No");
            return;
        }
    }

    for i in a..c.max(d) {
        // 移動範囲内に連続して岩があるならば不可
        // a > 0
        if vc[i as usize] == '#' && vc[i as usize - 1] == '#' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
