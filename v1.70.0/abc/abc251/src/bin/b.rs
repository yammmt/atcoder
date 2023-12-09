use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: u32,
        an: [u32; n],
    }

    let mut hs = HashSet::new();
    for &a in &an {
        if a <= w {
            hs.insert(a);
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            let cur = an[i] + an[j];
            if cur <= w {
                hs.insert(cur);
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let cur = an[i] + an[j] + an[k];
                if cur <= w {
                    hs.insert(cur);
                }
            }
        }
    }

    println!("{}", hs.len());
}
