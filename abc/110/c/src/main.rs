// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let vs = s.chars().collect::<Vec<char>>();
    let vt = t.chars().collect::<Vec<char>>();
    let mut vcnts = vec![0; 26];
    let mut vcntt = vec![0; 26];

    for c in &vs {
        vcnts[(*c as u8 - b'a') as usize] += 1;
    }
    for c in &vt {
        vcntt[(*c as u8 - b'a') as usize] += 1;
    }

    vcnts.sort();
    vcntt.sort();
    if vcnts == vcntt {
        println!("Yes");
    } else {
        println!("No");
    }
}
