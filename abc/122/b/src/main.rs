// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vs: Vec<char> = s.chars().collect();
    let mut ans = 0;
    let mut current = 0;
    for i in &vs {
        if *i == 'A' || *i == 'C' || *i == 'G' || *i == 'T' {
            current += 1;
        } else {
            ans = ans.max(current);
            current = 0;
        }
    }
    ans = ans.max(current);
    println!("{}", ans);
}
