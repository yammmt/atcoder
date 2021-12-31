use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: usize,
        q: usize,
        tncq: [(usize, usize, usize); q],
    }

    let mut ans = vec![0; c];
    let mut painted_h = HashSet::new();
    let mut painted_w = HashSet::new();
    for tnc in tncq.iter().rev() {
        if tnc.0 == 1 {
            if painted_w.contains(&tnc.1) {
                continue;
            }

            ans[tnc.2 - 1] += w - painted_h.len();
            painted_w.insert(tnc.1);
        } else {
            if painted_h.contains(&tnc.1) {
                continue;
            }

            ans[tnc.2 - 1] += h - painted_w.len();
            painted_h.insert(tnc.1);
        }
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
