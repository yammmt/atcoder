// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut abc: [u16; 3],
    }
    abc.sort();
    let ans = abc[2] * 10 + abc[1] + abc[0];
    println!("{}", ans);
}
