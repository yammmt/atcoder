// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        k: u16,
    }

    let mut sunuke = vec![0; n as usize];
    for _i in 0..k {
        input! {
            dd: u16,
            aa: [u16; dd as usize],
        }
        for j in 0..dd {
            sunuke[aa[j as usize] as usize - 1] += 1;
        }
    }
    println!("{}", sunuke.into_iter().filter(|&s| s == 0).count());
}
