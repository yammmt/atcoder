// -*- coding:utf-8-unix -*-

// 8.5 min

use proconio::input;

fn main() {
    input! {
        s: String,
        k: u64,
    }

    let vc = s.chars().collect::<Vec<char>>();
    let mut idx = 1;
    for c in &vc {
        if *c != '1' {
            println!("{}", c);
            return;
        } else if idx == k {
            println!("1");
            return;
        }
        idx += 1;
    }
}
