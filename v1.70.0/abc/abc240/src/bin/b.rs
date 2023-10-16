use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let hs: HashSet<usize> = HashSet::from_iter(an.iter().cloned());
    println!("{}", hs.len());
}
