// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut chars_cnt = vec![[0; 26]; n];
    for i in 0..n {
        let vc: Vec<char> = s[i].chars().collect();
        for j in 0..vc.len() {
            chars_cnt[i][(vc[j] as u8 - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        let mut current_min = 50 * 50 + 1;
        for j in 0..n {
            current_min = current_min.min(chars_cnt[j][i]);
            if current_min == 0 {
                continue;
            }
        }

        for _ in 0..current_min {
            print!("{}", (b'a' + i as u8) as char);
        }
    }
    println!("");
}
