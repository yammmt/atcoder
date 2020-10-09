// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: usize,
    }

    let vc = s.chars().collect::<Vec<char>>();
    let mut missing_num = 0;
    let mut moved = (0i64, 0i64);
    for c in &vc {
        match c {
            'L' => moved.0 -= 1,
            'R' => moved.0 += 1,
            'U' => moved.1 += 1,
            'D' => moved.1 -= 1,
            _ => missing_num += 1,
        }
    }
    if t == 1 {
        println!("{}", moved.0.abs() + moved.1.abs() + missing_num);
    } else {
        let fixed = moved.0.abs() + moved.1.abs();
        if missing_num <= fixed {
            println!("{}", fixed - missing_num);
        } else {
            if (missing_num - fixed) % 2 == 0 {
                println!("0");
            } else {
                println!("1");
            }
        }
    }
}
