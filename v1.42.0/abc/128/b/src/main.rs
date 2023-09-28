// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vsp = vec![];
    for i in 0..n {
        input! {
            s: String,
            p: u8,
        }
        vsp.push((s, p, i + 1));
    }
    vsp.sort_by(|a, b| {
        if a.0 != b.0 {
            a.cmp(b)
        } else {
            b.1.cmp(&a.1)
        }
    });
    for i in 0..n {
        println!("{}", vsp[i].2);
    }
}
