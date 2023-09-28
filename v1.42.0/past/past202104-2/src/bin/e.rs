use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut a = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        match *c {
            'L' => a.push_front(i + 1),
            'R' => a.push_back(i + 1),
            'A' => {
                if a.is_empty() {
                    println!("ERROR");
                } else {
                    println!("{}", a.pop_front().unwrap());
                }
            },
            'B' => {
                if a.len() <= 1 {
                    println!("ERROR");
                } else {
                    let first = a.pop_front().unwrap();
                    let second = a.pop_front().unwrap();
                    println!("{}", second);
                    a.push_front(first);
                }
            },
            'C' => {
                if a.len() <= 2 {
                    println!("ERROR");
                } else {
                    let first = a.pop_front().unwrap();
                    let second = a.pop_front().unwrap();
                    let third = a.pop_front().unwrap();
                    println!("{}", third);
                    a.push_front(second);
                    a.push_front(first);
                }
            },
            'D' => {
                if a.is_empty() {
                    println!("ERROR");
                } else {
                    println!("{}", a.pop_back().unwrap());
                }
            },
            'E' => {
                if a.len() <= 1 {
                    println!("ERROR");
                } else {
                    let first = a.pop_back().unwrap();
                    let second = a.pop_back().unwrap();
                    println!("{}", second);
                    a.push_back(first);
                }
            },
            'F' => {
                if a.len() <= 2 {
                    println!("ERROR");
                } else {
                    let first = a.pop_back().unwrap();
                    let second = a.pop_back().unwrap();
                    let third = a.pop_back().unwrap();
                    println!("{}", third);
                    a.push_back(second);
                    a.push_back(first);
                }
            },
            _ => unreachable!(),
        }
    }
}
