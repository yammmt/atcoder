use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        dn: [usize; n],
    }

    let mut hs = HashSet::new();
    for d in &dn {
        hs.insert(*d);
    }

    println!("{}", hs.len());
}
