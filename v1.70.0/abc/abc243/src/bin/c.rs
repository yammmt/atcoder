use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xyn: [(usize, usize); n],
        x: Chars,
    }

    let mut to_left = HashMap::new();
    let mut to_right = HashMap::new();
    for (i, &c) in x.iter().enumerate() {
        let mut x = xyn[i].0;
        let y = xyn[i].1;
        if c == 'L' {
            let e = to_left.entry(y).or_insert(x);
            *e = *e.max(&mut x);
        } else {
            let e = to_right.entry(y).or_insert(x);
            *e = *e.min(&mut x);
        }
    }

    for r in &to_right {
        let Some(x) = to_left.get(r.0) else { continue };
        if x > r.1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
