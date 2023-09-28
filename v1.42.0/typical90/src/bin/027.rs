use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let mut hs = HashSet::new();
    for (i, s) in sn.iter().enumerate() {
        if hs.contains(&s) {
            continue;
        }

        println!("{}", i + 1);
        hs.insert(s);
    }
}
