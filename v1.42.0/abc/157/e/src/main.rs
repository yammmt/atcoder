// Rust の平衡二分探索木 (BTreeSet) は滅多に使わず...

use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        q: usize,
    }

    let mut charsets = vec![BTreeSet::new(); 26];
    for (i, c) in s.iter().enumerate() {
        charsets[(*c as u8 - b'a') as usize].insert(i + 1);
    }

    for _ in 0..q {
        input! {
            a: usize,
        }
        if a == 1 {
            input! {
                i: usize,
                c: char,
            }
            charsets[(s[i - 1] as u8 - b'a') as usize].remove(&i);
            charsets[(c as u8 - b'a') as usize].insert(i);
            s[i - 1] = c;
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let mut ans = 0;
            for cs in &charsets {
                if cs.range(l..=r).next().is_some() {
                    ans += 1;
                }
            }
            println!("{}", ans);
        }
    }
    // println!("{:?}", charsets);
}
