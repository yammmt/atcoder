use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abcm: [(usize, usize, usize); m],
    }
    let abcm: Vec<(usize, usize, usize)> =
        abcm.iter().map(|a| (a.0 - 1, a.1 - 1, a.2 - 1)).collect();

    let mut ans = 0;
    for bitrow in 0..2u32.pow(n as u32) {
        let mut used = vec![false; n];
        for i in 0..n {
            if (bitrow >> i) & 0x01 == 1 {
                used[i] = true;
            }
        }

        let mut pass = true;
        for abc in &abcm {
            if used[abc.0] && used[abc.1] && used[abc.2] {
                pass = false;
                break;
            }
        }
        if !pass {
            continue;
        }

        let mut dangerous = HashSet::new();
        // println!("{:?}", used);
        for abc in &abcm {
            if !used[abc.0] && used[abc.1] && used[abc.2] {
                dangerous.insert(abc.0);
            }
            if used[abc.0] && !used[abc.1] && used[abc.2] {
                dangerous.insert(abc.1);
            }
            if used[abc.0] && used[abc.1] && !used[abc.2] {
                dangerous.insert(abc.2);
            }
        }

        ans = ans.max(dangerous.len());
    }

    println!("{}", ans);
}
