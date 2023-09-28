// -*- coding:utf-8-unix -*-

use proconio::input;

// 20 分で解けると 1260 くらい

fn main() {
    input! {
        s: String,
    }

    let vc = s.chars().collect::<Vec<char>>();
    let mut ans = 1;
    let mut prev = vec![vc[0]];
    let mut curr = vec![];
    for i in 1..vc.len() {
        curr.push(vc[i]);

        if curr != prev {
            prev = curr.clone();
            curr.clear();
            ans += 1;
        }
    }
    println!("{}", ans);
}
