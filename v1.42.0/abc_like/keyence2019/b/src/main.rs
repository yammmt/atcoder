// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if s.iter().collect::<String>() == "keyence" {
        println!("YES");
        return;
    }

    for i in 0..s.len() {
        for j in i..s.len() {
            if s.len() - (j - i + 1) != 7 {
                continue;
            }

            // println!("{}, {}", i, j);
            let mut vc = vec![];
            for k in 0..s.len() {
                if k < i || k > j {
                    vc.push(s[k]);
                }
            }
            // println!("vc: {:?}", vc);
            if vc.iter().collect::<String>() == "keyence" {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
