// -*- coding:utf-8-unix -*-

// https://atcoder.jp/contests/agc048/editorial/197

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let atcoder = String::from("atcoder");
    for _ in 0..t {
        input! {
            s: String,
        }

        if s > atcoder {
            println!("0");
            continue;
        }

        let vc = s.chars().collect::<Vec<char>>();
        for i in 0..vc.len() {
            if vc[i] != 'a' {
                if vc[i] <= 't' {
                    println!("{}", i);
                } else {
                    println!("{}", i - 1);
                }
                break;
            } else if i == vc.len() - 1 {
                println!("-1");
            }
        }
    }
}
