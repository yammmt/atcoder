// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        plan: [(i32, i32, i32); n],
    }

    let (mut t_from, mut x_from, mut y_from) = (0, 0, 0);
    for i in 0..n {
        let distance = (x_from - plan[i].1).abs() + (y_from - plan[i].2).abs();
        let margin = plan[i].0 - t_from - distance;
        match margin.cmp(&0) {
            Ordering::Less => {
                println!("No");
                return;
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                match margin % 2 {
                    0 => {},
                    _ => {
                        println!("No");
                        return;
                    },
                }
            }
        }
        t_from = plan[i].0;
        x_from = plan[i].1;
        y_from = plan[i].2;
    }
    println!("Yes");
}
