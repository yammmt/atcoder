use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let mut votes = HashMap::new();
    for s in &sn {
        let cnt = votes.entry(s).or_insert(0);
        *cnt += 1;
    }
    let mut ans = (0, "dummy");
    for (k, v) in &votes {
        if *v > ans.0 {
            ans.0 = *v;
            ans.1 = *k;
        }
    }
    println!("{}", ans.1);
}
