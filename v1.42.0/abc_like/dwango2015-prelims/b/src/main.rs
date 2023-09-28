// -*- coding:utf-8-unix -*-

// 7min

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vc = s.chars().collect::<Vec<char>>();

    let mut ans = 0;
    let mut streak = 0;
    for i in 0..vc.len() {
        if i == 0 {
            continue;
        }

        if vc[i] == '5' && vc[i - 1] == '2' {
            streak += 1;
            ans += streak;
        } else if vc[i] == '2' && vc[i - 1] == '5' {
            // do nothing
        } else {
            streak = 0;
        }
    }
    println!("{}", ans);
}
