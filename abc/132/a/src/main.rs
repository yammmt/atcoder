// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vs = s.chars().collect::<Vec<char>>();

    let mut chars_cnt = vec![0; 26];
    for c in &vs {
        chars_cnt[(*c as u8 - b'A') as usize] += 1;
    }
    chars_cnt.sort();
    if chars_cnt[25] == 2 && chars_cnt[24] == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
