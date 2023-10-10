use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        _m: usize,
    }
    let mut kn = Vec::with_capacity(n);
    let mut ankn = vec![];
    for _ in 0..n {
        input! {
            kk: usize,
            aa: [usize; kk],
        }
        kn.push(kk);
        ankn.push(aa.iter().cloned().collect::<HashSet<usize>>());
    }
    input! {
        p: usize,
        q: usize,
        bp: [usize; p],
    }

    let mut ans = 0;
    for ak in &ankn {
        let mut supports_num = 0;
        for b in &bp {
            if ak.contains(b) {
                supports_num += 1;
            }
        }
        if supports_num >= q {
            ans += 1;
        }
    }

    println!("{ans}");
}
