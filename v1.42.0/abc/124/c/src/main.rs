// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut vc = s.chars().collect::<Vec<char>>();

    let mut ans = 0;
    for i in 1..vc.len() {
        if vc[i] == vc[i - 1] {
            ans += 1;
            if vc[i] == '0' {
                vc[i] = '1';
            } else {
                vc[i] = '0';
            }
        }
    }
    println!("{}", ans);
}
