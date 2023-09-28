// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: i32,
        y: i32,
        mut xi: [i32; n],
        mut yi: [i32; m],
    }

    if x > y {
        println!("War");
        return;
    }

    xi.push(x);
    yi.push(y);
    xi.sort();
    yi.sort();
    if xi[n] >= yi[0] {
        println!("War");
    } else {
        println!("No War");
    }
}
