// -*- coding:utf-8-unix -*-

// 7min

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut vs = s.chars().collect::<Vec<char>>();
    vs.reverse();
    let mut idx = 0;
    while idx < vs.len() {
        if idx + 4 < vs.len() && vs[idx..idx + 5] == ['m', 'a', 'e', 'r', 'd'] {
            idx += 5;
        } else if idx + 4 < vs.len() && vs[idx..idx + 5] == ['e', 's', 'a', 'r', 'e'] {
            idx += 5;
        } else if idx + 6 < vs.len() && vs[idx..idx + 7] == ['r', 'e', 'm', 'a', 'e', 'r', 'd'] {
            idx += 7;
        } else if idx + 5 < vs.len() && vs[idx..idx + 6] == ['r', 'e', 's', 'a', 'r', 'e'] {
            idx += 6;
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
