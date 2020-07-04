// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut i: usize = 0;
    while i != s.len() {
        if s[i..].starts_with("dreameraser") {
            i += 11;
        } else if s[i..].starts_with("dreamerase") {
            i += 10;
        } else if s[i..].starts_with("dreamer") {
            i += 7;
        } else if s[i..].starts_with("eraser") {
            i += 6;
        } else if s[i..].starts_with("dream") || s[i..].starts_with("erase") {
            i += 5;
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
