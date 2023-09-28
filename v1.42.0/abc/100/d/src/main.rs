// -*- coding:utf-8-unix -*-

// 30min

use proconio::input;

fn score(xyzn: &[(i64, i64, i64)], m: usize) -> i64 {
    let mut ret = (0, 0, 0);
    xyzn.iter().take(m).for_each(|a| {
        ret.0 += a.0;
        ret.1 += a.1;
        ret.2 += a.2;
    });
    ret.0.abs() + ret.1.abs() + ret.2.abs()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xyzn: [(i64, i64, i64); n],
    }

    let mut ans = std::i64::MIN;
    let signs = [-1, 1];
    for a in &signs {
        for b in &signs {
            for c in &signs {
                xyzn.sort_unstable_by(|x, y| {
                    let aa = a * x.0 + b * x.1 + c * x.2;
                    let bb = a * y.0 + b * y.1 + c * y.2;
                    bb.cmp(&aa)
                });
                ans = ans.max(score(&xyzn, m));
            }
        }
    }

    println!("{}", ans);
}
