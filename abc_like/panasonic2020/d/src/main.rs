// -*- coding:utf-8-unix -*-

// 34min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    let mut vdq = VecDeque::new();
    vdq.push_back((vec!['a'], 'a'));
    while !vdq.is_empty() {
        let cv = vdq.pop_front().unwrap();
        if cv.0.len() == n {
            ans.push(cv.0.iter().collect::<String>());
            continue;
        }

        for c in 0..cv.1 as u8 - b'a'+ 2 {
            // println!("c: {}", c);

            let mut vc = cv.0.clone();
            vc.push((c + b'a') as char);
            vdq.push_back((vc, (cv.1.max((c + b'a') as char))));
        }
    }

    for s in &ans {
        println!("{}", s);
    }
}
