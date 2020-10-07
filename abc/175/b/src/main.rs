// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u64; n],
        // mut l: [u64; n],
    }

    // l.sort();
    // l.dedup();
    let mut ans = 0;
    for a in 0..l.len() {
        for b in a..l.len(){
            for c in b..l.len() {
                if l[a] == l[b] || l[b] == l[c] || l[c] == l[a] {
                    continue;
                }

                if l[a] + l[b] > l[c] && l[b] + l[c] > l[a] && l[c] + l[a] > l[b] {
                    // println!("{} {} {}", l[a], l[b], l[c]);
                    // println!("{} {} {}", a, b, c);
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
