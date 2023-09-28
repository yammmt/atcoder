// -*- coding:utf-8-unix -*-

use proconio::input;


fn main() {
    input! {
        s: String,
    }

    if s == "zyxwvutsrqponmlkjihgfedcba" {
        println!("-1");
        return;
    }

    let vs: Vec<char> = s.chars().collect();
    let mut vused = vec![false; 26];
    for i in 0..vs.len() {
        vused[(vs[i] as u8 - b'a') as usize] = true;
    }

    if vused.iter().all(|&a| a) {
        let mut vrevused = [false; 26];
        vrevused[(vs[vs.len() - 1] as u8 - b'a') as usize] = true;
        for i in (0..vs.len() - 1).rev() {
            // 後ろから見て昇順が成立しなくなったところの文字を変更する
            // 変更先は (後ろから見てのカウントで) 既出かつ最小のもの
            if vs[i] < vs[i + 1] {
                for j in 0..vrevused.len() {
                    if vrevused[j] && (j as u8 + b'a') > vs[i] as u8 {
                        println!("{}{}", vs[0..i].iter().collect::<String>(), (j as u8 + b'a') as char);
                        return;
                    }
                }
            }
            vrevused[(vs[i] as u8 -b'a') as usize] = true;
        }
    } else {
        for i in 0..vused.len() {
            if !vused[i] {
                println!("{}{}", s, (b'a' + i as u8) as char);
                return;
            }
        }
    }
}
