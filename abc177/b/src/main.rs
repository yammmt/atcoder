// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let vs: Vec<char> = s.chars().collect();
    let vt: Vec<char> = t.chars().collect();
    let mut saved = 0;
    for i in 0..vs.len() - vt.len() + 1 {
        let mut current = 0;
        // println!("i: {}", i);
        for j in 0..vt.len() {
            // println!("j: {}", j);
            if i + j >= vs.len() {
                continue;
            }

            if vs[i + j] == vt[j] {
                current += 1;
            } else {
                continue;
            }
        }
        // println!("streak: {}", current);
        saved = saved.max(current);
    }
    println!("{}", vt.len() - saved);
}
