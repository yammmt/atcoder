// 4min

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    if k > s.len() {
        println!("0");
        return;
    }

    let mut hs = HashSet::new();
    for i in 0..s.len() - k + 1 {
        let mut cur = vec![];
        s.iter().skip(i).take(k).for_each(|c| cur.push(*c));
        hs.insert(cur.iter().collect::<String>());
    }

    println!("{}", hs.len());
}
