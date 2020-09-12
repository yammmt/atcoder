// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        abcd: String,
    }

    let vch: Vec<i16> = abcd.chars().map(|c| c.to_digit(10).unwrap() as i16).collect();
    let vb = vec![vch[1], -vch[1]];
    let vc = vec![vch[2], -vch[2]];
    let vd = vec![vch[3], -vch[3]];
    for b in &vb {
        for c in &vc {
            for d in &vd {
                if vch[0] + b + c + d == 7 {
                    println!("{}{:+}{:+}{:+}=7", vch[0], b, c, d);
                    return;
                }
            }
        }
    }
}
