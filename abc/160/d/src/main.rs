// -*- coding:utf-8-unix -*-

// 25min

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
    }

    let mut vd = vec![0; n];
    for i in 1..n + 1 {
        for j in i + 1..n + 1 {
            let notused = j - i;
            let used = ((i as isize - x).abs() + (j as isize - y).abs() + 1) as usize;
            vd[notused.min(used)] += 1;
        }
    }

    for i in 1..n {
        println!("{}", vd[i]);
    }
}
