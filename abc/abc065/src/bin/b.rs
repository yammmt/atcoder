// 7min 1WA

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 1;
    let mut cur = an[0] - 1;
    let mut hs = HashSet::new();
    hs.insert(0);
    loop {
        if cur == 1 {
            println!("{}", ans);
            return;
        }

        if hs.contains(&cur) {
            println!("-1");
            return;
        }

        hs.insert(cur);
        cur = an[cur] - 1;
        ans += 1;
    }
}
