// -*- coding:utf-8-unix -*-

// 22min.

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        r: usize,
        c: usize,
        mut syx: (usize, usize),
        mut gyx: (usize, usize),
        mass: [Chars; r],
    }

    syx = (syx.0 - 1, syx.1 - 1);
    gyx = (gyx.0 - 1, gyx.1 - 1);
    let mut vdq = VecDeque::new();
    vdq.push_back((syx, 0));
    let mut visited = vec![vec![false; c]; r];
    visited[syx.0][syx.1] = true;
    while !vdq.is_empty() {
        let current = vdq.pop_front().unwrap();
        if current.0 == gyx {
            println!("{}", current.1);
            return;
        }
        if (current.0).0 > 0 {
            if mass[(current.0).0 - 1][(current.0).1] == '.' && !visited[(current.0).0 - 1][(current.0).1] {
                vdq.push_back((((current.0).0 - 1, (current.0).1), current.1 + 1));
                visited[(current.0).0 - 1][(current.0).1] = true;
            }
        }
        if (current.0).0 < r - 1 {
            if mass[(current.0).0 + 1][(current.0).1] == '.' && !visited[(current.0).0 + 1][(current.0).1] {
                vdq.push_back((((current.0).0 + 1, (current.0).1), current.1 + 1));
                visited[(current.0).0 + 1][(current.0).1] = true;
            }
        }
        if (current.0).1 > 0 {
            if mass[(current.0).0][(current.0).1 - 1] == '.' && !visited[(current.0).0][(current.0).1 - 1] {
                vdq.push_back((((current.0).0, (current.0).1 - 1), current.1 + 1));
                visited[(current.0).0][(current.0).1 - 1] = true;
            }
        }
        if (current.0).1 < c - 1 {
            if mass[(current.0).0][(current.0).1 + 1] == '.' && !visited[(current.0).0][(current.0).1 + 1] {
                vdq.push_back((((current.0).0, (current.0).1 + 1), current.1 + 1));
                visited[(current.0).0][(current.0).1 + 1] = true;
            }
        }
    }
}
