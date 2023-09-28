use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut tn = vec![];
    let mut ank = vec![];
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            ak: [usize; k],
        }
        tn.push(t);
        ank.push(ak);
    }

    let mut required = vec![false; n];
    let mut vdq = VecDeque::new();
    vdq.push_back(n - 1);
    required[n - 1] = true;
    while let Some(cur) = vdq.pop_front() {
        for a in &ank[cur] {
            if !required[a - 1] {
                required[a - 1] = true;
                vdq.push_back(a - 1);
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if required[i] {
            ans += tn[i];
        }
    }

    println!("{}", ans);
}
