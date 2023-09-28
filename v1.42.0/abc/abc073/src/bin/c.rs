use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    let mut hs = HashSet::new();
    for a in &an {
        match hs.contains(a) {
            true => hs.remove(&a),
            false => hs.insert(a),
        };
    }
    println!("{}", hs.len());
}
