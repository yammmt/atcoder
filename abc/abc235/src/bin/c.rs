use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        an: [usize; n],
        xkq: [(usize, usize); q],
    }

    let mut hm = HashMap::new();
    for (i, a) in an.iter().enumerate() {
        let cur = hm.entry(a).or_insert_with(|| vec![]);
        cur.push(i + 1);
    }

    for xk in &xkq {
        match hm.get(&xk.0) {
            Some(v) => {
                if xk.1 - 1 < v.len() {
                    println!("{}", v[xk.1 - 1]);
                } else {
                    println!("-1");
                }
            }
            None => println!("-1"),
        }
    }
}
