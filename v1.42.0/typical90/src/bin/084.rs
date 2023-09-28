use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: u64,
        s: Chars,
    }

    let mut o = VecDeque::new();
    let mut x = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        if *c == 'o' {
            o.push_back(i as u64);
        } else {
            x.push_back(i as u64);
        }
    }

    let mut ans = 0u64;
    for c in &s {
        // println!("o: {:?}", o);
        // println!("x: {:?}", x);
        if *c == 'o' {
            o.pop_front();
            if !x.is_empty() {
                let pos = x.pop_front().unwrap();
                ans += n - pos;
                x.push_front(pos);
            }
        } else {
            x.pop_front();
            if !o.is_empty() {
                let pos = o.pop_front().unwrap();
                ans += n - pos;
                o.push_front(pos);
            }
        }
        // println!("  {}", ans);
    }

    println!("{}", ans);
}
