use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        bm: [i64; m],
    }
    let mut hsa = HashSet::new();
    let mut hsb = HashSet::new();
    for a in &an {
        hsa.insert(a);
    }
    for b in &bm {
        hsb.insert(b);
    }

    let mut ans = vec![];
    for a in &an {
        if !hsb.contains(a) {
            ans.push(a);
        }
    }
    for b in &bm {
        if !hsa.contains(b) {
            ans.push(b);
        }
    }
    ans.sort_unstable();

    for (i, a) in ans.iter().enumerate() {
        println!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
