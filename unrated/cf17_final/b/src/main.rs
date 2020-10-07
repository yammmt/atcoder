// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut c_cnt = vec![0; 3];
    for c in &vc {
        c_cnt[(*c as u8 - b'a') as usize] += 1;
    }
    c_cnt.sort();
    if c_cnt[2] - c_cnt[0] < 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
