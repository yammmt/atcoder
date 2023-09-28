// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
        s: u32,
    }

    if n == 1 {
        if k == 0 {
            if s == 10u32.pow(9) {
                println!("1");
            } else {
                println!("{}", s + 1);
            }
        } else {
            println!("{}", s);
        }
        return;
    }

    for i in 0..n {
        if i < k {
            print!("{}", s);
        } else {
            if s == 10u32.pow(9) {
                print!("1");
            } else {
                print!("{}", s + 1);
            }
        }

        if i != n - 1 {
            print!(" ");
        } else {
            println!("");
        }
    }
}
