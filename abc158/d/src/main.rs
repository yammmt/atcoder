// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        s: String,
        q: u64,
    }

    let mut vs: Vec<char> = s.chars().collect();
    let mut is_reversed = false;
    let mut vhead: Vec<char> = vec!();
    let mut vtail: Vec<char> = vec!();
    for _i in 0..q {
        input! {
            c0: char,
        }

        // println!("{:?}", query);
        if c0 == '1' {
            is_reversed = !is_reversed;
        } else {
            input! {
                c1: char,
                c2: char,
            }
            if is_reversed {
                if c1 == '1' {
                    vtail.push(c2);
                } else {
                    vhead.push(c2);
                }
            } else {
                if c1 == '1' {
                    vhead.push(c2);
                } else {
                    vtail.push(c2);
                }
            }
        }
    }

    vhead.reverse();
    vhead.append(&mut vs);
    let mut vs =vhead;
    vs.append(&mut vtail);
    if is_reversed {
        vs.reverse();
    }
    println!("{}", vs.into_iter().collect::<String>());
}
