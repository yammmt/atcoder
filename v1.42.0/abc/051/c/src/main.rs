// -*- coding:utf-8-unix -*-

// 11 min.

use proconio::input;

fn main() {
    input! {
        sxy: (i32, i32),
        txy: (i32, i32),
    }
    let mut vans = vec![];
    for _ in 0..(txy.1 - sxy.1) {
        vans.push('U');
    }
    for _ in 0..(txy.0 - sxy.0) {
        vans.push('R');
    }
    for _ in 0..(txy.1 - sxy.1) {
        vans.push('D');
    }
    for _ in 0..(txy.0 - sxy.0) {
        vans.push('L');
    }

    vans.push('L');
    for _ in 0..(txy.1 - sxy.1) + 1 {
        vans.push('U');
    }
    for _ in 0..(txy.0 - sxy.0) + 1 {
        vans.push('R');
    }
    vans.push('D');
    vans.push('R');
    for _ in 0..(txy.1 - sxy.1) + 1 {
        vans.push('D');
    }
    for _ in 0..(txy.0 - sxy.0) + 1{
        vans.push('L');
    }
    vans.push('U');

    for c in &vans {
        print!("{}", *c);
    }
    println!("");
}
