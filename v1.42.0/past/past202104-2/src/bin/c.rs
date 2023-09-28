use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        _m: usize,
    }
    let mut kn = vec![];
    let mut ankn = vec![];
    for _ in 0..n {
        input! {
            k: usize,
        }
        kn.push(k);
        input! {
            va: [usize; k],
        }
        ankn.push(va);
    }
    input! {
        p: usize,
        q: usize,
        bp: [usize; p],
    }

    let mut hs = HashSet::new();
    for b in &bp {
        hs.insert(b);
    }

    let mut ans = 0;
    for an in &ankn {
        let mut cur = 0;
        for a in an {
            if hs.contains(&a) {
                cur += 1;
            }
        }
        if cur >= q {
            ans += 1;
        }
    }
    println!("{}", ans);
}
