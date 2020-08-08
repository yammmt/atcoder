// -*- coding:utf-8-unix -*-

use proconio::input;

fn is_kaibun(s: &str) -> bool {
    let ss: String = s.chars().rev().collect();
    s == ss
}

fn main() {
    input! {
        s: String,
    }

    if is_kaibun(&s) &&  is_kaibun(&s[0..((s.len() - 1) / 2)]) && is_kaibun(&s[(s.len() + 2)/2..s.len()]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
