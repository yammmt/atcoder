// -*- coding:utf-8-unix -*-

use permutohedron::heap_recursive;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut data: Vec<usize> = (1..n+1).collect();
    let mut vord = vec![];
    heap_recursive(&mut data, |p| {
        vord.push(p.to_vec());
    });
    vord.sort();
    let mut a = 0;
    let mut b = 0;
    for i in 0..vord.len() {
        if vord[i] == p {
            a = (i + 1) as i32;
        }
        if vord[i] == q {
            b = (i + 1) as i32;
        }
    }
    println!("{}", (a - b).abs());
}
