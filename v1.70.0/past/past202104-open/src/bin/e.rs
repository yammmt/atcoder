use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut vdq = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        let ii = i + 1;
        match *c {
            'L' => vdq.push_front(ii),
            'R' => vdq.push_back(ii),
            'A' => {
                if let Some(v) = vdq.pop_front() {
                    println!("{v}");
                } else {
                    println!("ERROR");
                }
            }
            'B' => {
                if vdq.len() < 2 {
                    println!("ERROR");
                } else {
                    let v1 = vdq.pop_front().unwrap();
                    let v2 = vdq.pop_front().unwrap();
                    vdq.push_front(v1);
                    println!("{v2}");
                }
            }
            'C' => {
                if vdq.len() < 3 {
                    println!("ERROR");
                } else {
                    let v1 = vdq.pop_front().unwrap();
                    let v2 = vdq.pop_front().unwrap();
                    let v3 = vdq.pop_front().unwrap();
                    vdq.push_front(v2);
                    vdq.push_front(v1);
                    println!("{v3}");
                }
            }
            'D' => {
                if let Some(v) = vdq.pop_back() {
                    println!("{v}");
                } else {
                    println!("ERROR");
                }
            }
            'E' => {
                if vdq.len() < 2 {
                    println!("ERROR");
                } else {
                    let v1 = vdq.pop_back().unwrap();
                    let v2 = vdq.pop_back().unwrap();
                    vdq.push_back(v1);
                    println!("{v2}");
                }
            }
            'F' => {
                if vdq.len() < 3 {
                    println!("ERROR");
                } else {
                    let v1 = vdq.pop_back().unwrap();
                    let v2 = vdq.pop_back().unwrap();
                    let v3 = vdq.pop_back().unwrap();
                    vdq.push_back(v2);
                    vdq.push_back(v1);
                    println!("{v3}");
                }
            }
            _ => unreachable!(),
        }
    }
}
