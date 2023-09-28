// -*- coding:utf-8-unix -*-

use permutohedron::heap_recursive;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut xy = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        xy.push((x, y));
    }

    let mut sumf = 0f64;
    let mut sump = 0;
    let mut data: Vec<usize> = (0..n).collect();
    heap_recursive(&mut data, |p| {
        // println!("{:?}", p);
        for i in 1..n {
            sumf += (((xy[p[i]].0 - xy[p[i - 1]].0).pow(2) + (xy[p[i]].1 - xy[p[i - 1]].1).pow(2)) as f64).sqrt();
        }
        sump += 1;
    });
    println!("{}", sumf / (sump as f64));
}
