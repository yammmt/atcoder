// -*- coding:utf-8-unix -*-

// 8min.

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let vs = s.chars().collect::<Vec<char>>();
    let vt = t.chars().collect::<Vec<char>>();

    let mut vscnt = vec![0; 26];
    for c in &vs {
        vscnt[(*c as u8 - b'a') as usize] += 1;
    }
    let mut s2 = String::new();
    for i in 0..vscnt.len() {
        // まともなものならまとめて `push` すべき
        for _ in 0..vscnt[i] {
            s2.push((b'a' + i as u8) as char);
        }
    }

    let mut vtcnt = vec![0; 26];
    for c in &vt {
        vtcnt[(*c as u8 - b'a') as usize] += 1;
    }
    let mut t2 = String::new();
    for i in 0..vtcnt.len() {
        for _ in 0..vtcnt[vtcnt.len() - i - 1] {
            t2.push((b'z' - i as u8) as char);
        }
    }

    if s2 < t2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
