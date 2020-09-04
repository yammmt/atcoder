// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;
    for i in s {
        match i.as_ref() {
            "AC" => ac += 1,
            "WA" => wa += 1,
            "TLE" => tle += 1,
            "RE" => re += 1,
            _ => {},
        }
    }
    println!("AC x {}\nWA x {}\nTLE x {}\nRE x {}", ac, wa, tle, re);
}
