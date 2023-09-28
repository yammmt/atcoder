use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        sn: [String; n],
        tm: [String; m],
    }

    let mut stoppable = HashSet::new();
    for t in tm {
        stoppable.insert(t);
    }

    for s in &sn {
        println!("{}", if stoppable.contains(s) { "Yes" } else { "No" })
    }
}
